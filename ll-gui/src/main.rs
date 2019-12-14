use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
mod ui;
mod utils;
pub mod tasks;
mod state;
pub mod consts;
pub mod types {
    pub use super::ui::notebook::updates::Updates;
    pub use super::ui::notebook::output::Output;
    pub use super::state::State;
    pub type AMState = std::sync::Arc<std::sync::Mutex<super::state::State>>;
}
use std::sync::{Arc, Mutex};
use library_loader_core::{
    Config,
    is_debug
};
use ui::Ui;

fn main() {

    let config = Config::load();

    let state: types::AMState = Arc::new(Mutex::new(state::State {
        save_login_info: config.profile.username.len() > 0 && config.profile.password.len() > 0,
        logged_in: config.profile.username.len() > 0 && config.profile.password.len() > 0,
        config: config
    }));

    // Load resources
    utils::load_resources();

    // Create application
    let application =
    gtk::Application::new(Some("net.olback.library-loader"), Default::default())
    .unwrap();

    let state_clone = Arc::clone(&state);
    application.connect_activate(move |app| {

        // Build ui
        let ui = Ui::build(app, &state_clone);

        // If already logged in, show the 'watch' tab.
        let state_clone_lock = state_clone.lock().unwrap();
        if state_clone_lock.logged_in {
            ui.notebook.inner.set_current_page(Some(1));
        }
        drop(state_clone_lock);

        // If not in debug mode, check for updates.
        if !is_debug!() {
            tasks::check_updates(&ui.notebook.updates);
        }

    });

    // Run app
    application.run(&args().collect::<Vec<_>>());

    // Save config on exit
    let state_lock = state.lock().unwrap();
    let deref = &*state_lock;
    match utils::save_config(deref) {
        Ok(b) => {
            if b {
                println!("Config saved");
            }
        },
        Err(e) => eprintln!("{}", e)
    };
    drop(state_lock);

}
