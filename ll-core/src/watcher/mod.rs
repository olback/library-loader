use {
    crate::{
        config::Config, cse::CSE, epw::Epw, error::Result, format::Format, log_error, log_if_error,
        log_info, logger::Logger,
    },
    event::WatcherEvent,
    notify::{
        event::CreateKind as NotifyCreateKind, EventKind as NotifyEventKind,
        Watcher as NotifyWatcher,
    },
    std::{
        ffi::OsString,
        path::PathBuf,
        sync::{mpsc, Arc},
        thread::{self, JoinHandle},
    },
};

mod event;

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
            }, notify::Config::default())?;

        let token = self.token.clone();
        let formats = Arc::clone(&self.formats);
        let loggers = Arc::clone(&self.loggers);
        let jh = thread::spawn(move || loop {
            match rx.recv() {
                Ok(WatcherEvent::NotifyResult(Ok(event))) => {
                    // log_info!(&*loggers, format!("{:#?}", event));
                    match event.kind {
                        NotifyEventKind::Create(NotifyCreateKind::File) => {
                            // println!("evt: {:#?}", event);
                            for file in event.paths {
                                if file.extension().map(|e| e.to_ascii_lowercase())
                                    == Some(OsString::from("zip"))
                                {
                                    log_info!(&*loggers, format!("Detected {:?}", file));
                                    let token = token.clone();
                                    let formats = Arc::clone(&formats);
                                    let loggers_clone = Arc::clone(&loggers);
                                    // uuuh
                                    std::thread::sleep(std::time::Duration::from_millis(100));
                                    match (move || -> Result<()> {
                                        let epw = Epw::from_file(file)?;
                                        for res in CSE::new(token, formats).get(epw)? {
                                            match res.save() {
                                                Ok(save_path) => {
                                                    log_info!(
                                                        &*loggers_clone,
                                                        format!("Saved to {:?}", save_path)
                                                    )
                                                }
                                                Err(e) => {
                                                    log_error!(&*loggers_clone, e)
                                                }
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        Ok(()) => {
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
        match self.thread.take() {
            Some((jh, tx, mut w)) => {
                log_if_error!(&*self.loggers, w.unwatch(self.watch_path.as_path()));
                log_if_error!(&*self.loggers, tx.send(WatcherEvent::Stop));
                log_if_error!(&*self.loggers, jh.join());
                log_info!(
                    &*self.loggers,
                    format!("Stopped watching {:?}", self.watch_path)
                );
            }
            None => {}
        }
    }
}
