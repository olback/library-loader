mod console;

pub use console::ConsoleLogger;

pub trait Logger: Send + Sync {
    fn trace(&self, data: String);
    fn info(&self, data: String);
    fn warn(&self, data: String);
    fn error(&self, data: String);
}

#[macro_export]
macro_rules! log_trace {
    ($loggers:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        for l in $loggers {
            l.trace(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($loggers:expr, $($arg:tt)*) => {
        for l in $loggers {
            l.info(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($loggers:expr, $($arg:tt)*) => {
        for l in $loggers {
            l.warn(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($loggers:expr, $($arg:tt)*) => {
        for l in $loggers {
            l.error(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_if_error {
    ($loggers:expr, $res:expr) => {
        if let Err(e) = $res {
            for l in $loggers {
                l.error(format!("{:?}", e))
            }
        }
    };
}
