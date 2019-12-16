use library_loader_core::Config;
use crate::tasks::WatcherHandler;

type LogTX = glib::Sender<String>;
type LogRX = glib::Receiver<String>;

#[derive(Debug)]
pub struct State {
    pub save_login_info: bool,
    pub logged_in: bool,
    pub config: Config,
    log: (LogTX, Option<LogRX>),
    watcher: Option<WatcherHandler>
}

impl State {

    pub fn new() -> Self {

        let config = Config::load();
        let (tx, rx) = glib::MainContext::channel::<String>(glib::PRIORITY_DEFAULT);

        Self {
            save_login_info: config.profile.username.len() > 0 && config.profile.password.len() > 0,
            logged_in: config.profile.username.len() > 0 && config.profile.password.len() > 0,
            config: config,
            log: (tx, Some(rx)),
            watcher: None
        }

    }

    pub fn get_log_tx(&self) -> LogTX {
        self.log.0.clone()
    }

    pub fn get_log_rx(&mut self) -> Option<LogRX> {
        self.log.1.take()
    }

    pub fn watcher_running(&self) -> bool {
        self.watcher.is_some()
    }

    pub fn set_watcher(&mut self, w: WatcherHandler) {
        self.watcher = Some(w)
    }

    pub fn stop_watcher(&mut self) {

        match self.watcher.take() {
            Some(w) => w.stop(),
            None => {}
        };

    }

}
