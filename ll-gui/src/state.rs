use library_loader_core::Config;

#[derive(Debug)]
pub struct State {
    pub save_login_info: bool,
    pub logged_in: bool,
    pub config: Config
}
