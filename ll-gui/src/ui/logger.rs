use {
    gtk::glib::{self, Receiver, Sender},
    ll_core::Logger,
};

type Message = String;

#[derive(Clone)]
pub struct GuiLogger {
    tx: Sender<Message>,
}

impl GuiLogger {
    pub fn new() -> (Receiver<Message>, Box<Self>) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        (rx, Box::new(Self { tx }))
    }
}

impl Logger for GuiLogger {
    fn trace(&self, data: String) {
        drop(self.tx.send(format!("[TRACE] {}", data)))
    }
    fn info(&self, data: String) {
        drop(self.tx.send(format!("[INFO] {}", data)))
    }
    fn warn(&self, data: String) {
        drop(self.tx.send(format!("[WARNING] {}", data)))
    }
    fn error(&self, data: String) {
        drop(self.tx.send(format!("[ERROR] {}", data)))
    }
}
