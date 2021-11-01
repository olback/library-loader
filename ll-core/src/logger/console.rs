use std::fmt::Display;

pub struct ConsoleLogger;

impl ConsoleLogger {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl super::Logger for ConsoleLogger {
    fn trace(&self, data: String) {
        println!("{}", data)
    }
    fn info(&self, data: String) {
        println!("{}", data)
    }
    fn warn(&self, data: String) {
        eprintln!("{}", data)
    }
    fn error(&self, data: String) {
        eprintln!("{}", data)
    }
}
