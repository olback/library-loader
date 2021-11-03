pub enum WatcherEvent {
    NotifyResult(notify::Result<notify::Event>),
    Stop,
}
