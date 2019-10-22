// use crossbeam_channel::unbounded;
use crossbeam_channel;
use notify::{
    self,
    Watcher as NotifyWatcher
};
use std::{
    time::Duration,
    path::PathBuf
};
use super::error::{LLResult, LLError};
use super::cse::CSE;

struct ThreadMessage {
    stop: bool,
    // event: Option<notify::Event>
}

// type ToTM = crossbeam_channel::Sender<Result<notify::Event, notify::Error>>;
// impl From<ToTM> for ThreadMessage {
//     fn from(from: ToTM) -> Self {
//         Self {
//             stop: false,
//             // event: Some(from)
//             event: from
//         }
//     }
// }

pub struct Watcher {
    thread: std::thread::JoinHandle<()>,
    path: PathBuf,
    watcher: notify::RecommendedWatcher
}

impl Watcher {

    pub fn start<P: Into<PathBuf>>(path: P, cse: CSE) -> LLResult<Self> {

        let p = path.into();
        // let (tx, rx) = crossbeam_channel::unbounded();
        // let w: notify::RecommendedWatcher = notify::Watcher::new(tx, Duration::from_secs(2))?;

        let t_path = p.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let w: notify::RecommendedWatcher = notify::Watcher::new(tx, Duration::from_secs(2))?;

        let t_handle = std::thread::spawn(move || {

            println!("Watching {:?}...", t_path);

            loop {

                match rx.recv() {
                    Ok(event) => {
                        break;
                    },
                    Err(e) => {
                        panic!("{:#?}", e);
                    }
                };

                // match rx.recv() {
                //     Ok(event) => {
                //         match event {
                //             Ok(e) => Self::handle_event(e.clone()),
                //             Err(e) => eprintln!("{:#?}", e)
                //         }
                //     },
                //     Err(e) => eprintln!("{:#?}", e)
                // };

            }

        });

        Ok(Self {
            thread: t_handle,
            path: p,
            watcher: w
        })

    }

    pub fn stop(mut self) -> LLResult<()> {

        println!("Should stop :(");

        match self.thread.join() {
            Ok(_) => {},
            Err(e) => eprintln!("{:#?}", e)
        }

        match &self.watcher.unwatch(&self.path) {
            Ok(v) => Ok(*v),
            Err(e) => Err(LLError::new(format!("{}", e)))
        }

    }

    fn handle_event(/*evt: notify::Event*/) -> () {

        // let event = evt.clone();
        // let path = &event.paths[0];

        // let file_ext = match event.kind {
        //     notify::EventKind::Create(_) => {
        //         let p = PathBuf::from(path);
        //         match p.extension() {
        //             Some(ext_os_str) => {
        //                 match ext_os_str.to_str() {
        //                     Some(ext_str) => Some(String::from(ext_str)),
        //                     None => None
        //                 }
        //             },
        //             None => None
        //         }
        //     },
        //     _ => None
        // };

        // if file_ext.is_some() {
        //     match file_ext.unwrap().as_str() {
        //         "zip" => {
        //             println!("Detected a zip file");
        //         },
        //         "epw" => {
        //             println!("Detected a epw file");
        //         },
        //         _ => {
        //             println!("Ignoring {:?}", path);
        //         }
        //     }
        // }

    }

}


