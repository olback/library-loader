#[macro_export]
macro_rules! new_err {
    ($e:expr) => {
        // Error::new($e, std::file!(), std::line!())
        LLError::new(format!("{}", $e), std::file!(), std::line!())
    };
}

#[macro_export]
macro_rules! is_debug {
    () => {
        if cfg!(debug_assertions) {
            true
        } else {
            std::env::var("LL_DEBUG").is_ok()
        }
    };
}

#[macro_export]
macro_rules! impl_from {
    ($t:ty) => {
        impl From<$t> for LLError {
            fn from(err: $t) -> LLError {
                super::new_err!(format!("{}", err))
            }
        }
    };
}
