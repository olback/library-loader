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
