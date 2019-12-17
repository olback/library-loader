use library_loader_core::{CSE, Watcher, LLResult, TX, NotifyError};
use crate::{
    utils::safe_lock,
    types::AMState
};

#[derive(Debug)]
pub struct WatcherHandler {
    channel: TX,
    join_handle: std::thread::JoinHandle<LLResult<()>>
}

impl WatcherHandler {

    pub fn stop(self) {
        println!("Stopping watcher tread");
        self.channel.send(Err(NotifyError::generic("stop"))).unwrap();
        match self.join_handle.join().unwrap() {
            Ok(_) => { /* Clean exit */ },
            Err(_) => { /* Error exit */ }
        };
    }

}

pub fn watcher(state: &AMState) -> LLResult<()> {

    let (config, log_tx) = safe_lock(&state, |lock| {
        (lock.config.clone(), lock.get_log_tx())
    });

    println!("Starting watcher thread");

    let cse = CSE::new(&config);
    let mut w = Watcher::new(config.settings.watch_path.unwrap(), cse, log_tx)?;
    let c = w.get_tx();

    let jh = std::thread::spawn::<_, LLResult<()>>(move || {
        w.start()
    });

    safe_lock(&state, |lock| {
        lock.set_watcher(WatcherHandler {
            channel: (&c).clone(),
            join_handle: jh
        });
    });

    Ok(())

}
