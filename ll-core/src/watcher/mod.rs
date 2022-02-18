use {
    crate::{
        config::Config, cse::CSE, epw::Epw, error::Result, format::Format, log_error, log_if_error,
        log_info, log_trace, logger::Logger,
    },
    event::WatcherEvent,
    notify::{
        event::{
            CreateKind as NotifyCreateKind, ModifyKind as NotifyModifyKind,
            RenameMode as NotifyRenameMode,
        },
        EventKind as NotifyEventKind, Watcher as NotifyWatcher,
    },
    std::{
        ffi::OsString,
        path::{Path, PathBuf},
        sync::{mpsc, Arc},
        thread::{self, JoinHandle},
        time::{Duration, Instant},
    },
};

mod event;

const TIME_EVENT_IGNORE: Duration = Duration::from_secs(5);

pub struct Watcher {
    token: String,
    watch_path: PathBuf,
    formats: Arc<Vec<Format>>,
    loggers: Arc<Vec<Box<dyn Logger>>>,
    thread: Option<(
        JoinHandle<()>,
        mpsc::Sender<WatcherEvent>,
        notify::RecommendedWatcher,
    )>,
    recursive: bool,
    ignore_temp: bool,
}

impl Watcher {
    pub fn new(config: Config, loggers: Vec<Box<dyn Logger>>) -> Result<Self> {
        Ok(Self {
            token: config.profile.token(),
            watch_path: PathBuf::from(shellexpand::full(&config.settings.watch_path)?.as_ref()),
            formats: Arc::new(config.formats()?),
            loggers: Arc::new(loggers),
            thread: None,
            recursive: config.settings.recursive,
            ignore_temp: config.settings.ignore_temp,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let ntx = tx.clone();

        let loggers = Arc::clone(&self.loggers);
        let mut w: notify::RecommendedWatcher =
            notify::Watcher::new(move |evt| match ntx.send(WatcherEvent::NotifyResult(evt)) {
                Ok(_) => {}
                Err(e) => log_error!(&*loggers, format!("{:?}", e)),
            })?;

        let token = self.token.clone();
        let formats = Arc::clone(&self.formats);
        let loggers = Arc::clone(&self.loggers);
        let ignore_temp = self.ignore_temp;
        let mut last_file: Option<(PathBuf, Instant)> = None;
        let jh = thread::spawn(move || loop {
            match rx.recv() {
                Ok(WatcherEvent::NotifyResult(Ok(event))) => {
                    // log_info!(&*loggers, format!("{:#?}", event));
                    match event.kind {
                        NotifyEventKind::Modify(NotifyModifyKind::Name(NotifyRenameMode::Both)) => {
                            if event.paths.len() == 2 {
                                log_trace!(&*loggers, format!("Modify paths: {:#?}", event.paths));
                                if check_process_file(&event.paths[1], ignore_temp, &last_file) {
                                    log_trace!(&*loggers, "NotifyEventKind::Modify");
                                    log_info!(&*loggers, format!("Detected {:?}", &event.paths[1]));
                                    match process(&loggers, &event.paths[1], &token, &formats) {
                                        Ok(()) => {
                                            last_file =
                                                Some((event.paths[1].clone(), Instant::now()));
                                            log_info!(&*loggers, "Done");
                                        }
                                        Err(e) => {
                                            log_error!(&*loggers, format!("{:?}", e));
                                        }
                                    }
                                }
                            }
                        }
                        NotifyEventKind::Create(NotifyCreateKind::File) => {
                            for file in event.paths {
                                if check_process_file(&file, ignore_temp, &last_file) {
                                    log_trace!(&*loggers, "NotifyEventKind::Create");
                                    log_info!(&*loggers, format!("Detected {:?}", file));
                                    // uuuh
                                    std::thread::sleep(std::time::Duration::from_millis(100));
                                    match process(&loggers, &file, &token, &formats) {
                                        Ok(()) => {
                                            last_file = Some((file, Instant::now()));
                                            log_info!(&*loggers, "Done");
                                        }
                                        Err(e) => {
                                            log_error!(&*loggers, format!("{:?}", e));
                                        }
                                    }
                                }
                            }
                            // log_info!(&*loggers, format!("{:#?}", event));
                        }
                        _ => {}
                    }
                }
                Ok(WatcherEvent::NotifyResult(Err(error))) => {
                    log_error!(&*loggers, format!("{:#?}", error))
                }
                Ok(WatcherEvent::Stop) => break,
                Err(_recv_error) => {
                    log_error!(&*loggers, "TX has gone away")
                }
            }
        });

        w.watch(
            self.watch_path.as_path(),
            if self.recursive {
                notify::RecursiveMode::Recursive
            } else {
                notify::RecursiveMode::NonRecursive
            },
        )?;

        self.thread = Some((jh, tx, w));

        log_info!(
            &*self.loggers,
            format!("Started watching {:?}", self.watch_path)
        );

        log_info!(&*self.loggers, "Active formats:");
        for f in &*self.formats {
            log_info!(
                &*self.loggers,
                format!("\t{} => {:?}", f.ecad, f.output_path)
            )
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some((jh, tx, mut w)) = self.thread.take() {
            log_if_error!(&*self.loggers, w.unwatch(self.watch_path.as_path()));
            log_if_error!(&*self.loggers, tx.send(WatcherEvent::Stop));
            log_if_error!(&*self.loggers, jh.join());
            log_info!(
                &*self.loggers,
                format!("Stopped watching {:?}", self.watch_path)
            );
        }
    }
}

fn process(
    loggers: &Arc<Vec<Box<dyn Logger>>>,
    file: &Path,
    token: &str,
    formats: &Arc<Vec<Format>>,
) -> Result<()> {
    let epw = Epw::from_file(file)?;
    for res in CSE::new(String::from(token), Arc::clone(formats)).get(epw)? {
        match res.save() {
            Ok(save_path) => {
                log_info!(&**loggers, format!("Saved to {:?}", save_path))
            }
            Err(e) => {
                log_error!(&**loggers, e)
            }
        }
    }
    Ok(())
}

fn check_process_file(
    path: &Path,
    ignore_temp: bool,
    last_file: &Option<(PathBuf, Instant)>,
) -> bool {
    const TEMP_FILES: &[&str] = &[".crdownload", ".part", ".download", ".wkdownload"];
    let same_as_last = {
        if let Some((f, i)) = last_file {
            f == path && Instant::now() - *i < TIME_EVENT_IGNORE
        } else {
            false
        }
    };
    let is_zip = path.extension().map(|e| e.to_ascii_lowercase()) == Some(OsString::from("zip"));
    let is_temp = path.components().any(|c| {
        let part = c.as_os_str().to_string_lossy();
        !TEMP_FILES.iter().all(|p| !part.contains(*p))
    });

    if ignore_temp {
        // Must be zip and not a temp file
        !same_as_last && is_zip && !is_temp
    } else {
        // Must be zip, may or may not be a temp file
        !same_as_last && is_zip
    }
}

#[cfg(test)]
mod tests {

    use {super::check_process_file, std::path::PathBuf};

    #[test]
    fn check_process_file_test() {
        let temp_paths = &[
            PathBuf::from("/a/b/c.zip.crdownload"),
            PathBuf::from("/a/b/c.zip.part"),
            PathBuf::from("/a/b/c.zip.download"),
            PathBuf::from("/a/b/c.zip.crdownload/c.zip"),
            PathBuf::from("/a/b/c.zip.part/c.zip"),
            PathBuf::from("/a/b/c.zip.download/c.zip"),
        ];

        for p in temp_paths {
            assert!(check_process_file(p, true, &None) == false)
        }

        let safari_path = PathBuf::from("/a/b/c.zip.download/c.zip");
        assert!(check_process_file(&safari_path, true, &None) == false);
        assert!(check_process_file(&safari_path, false, &None) == true);

        let real_path = PathBuf::from("/a/b/c.zip");
        assert!(check_process_file(&real_path, true, &None) == true);
        assert!(check_process_file(&real_path, false, &None) == true);
    }
}
