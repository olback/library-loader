use gio::prelude::*;
use std::env::args;
mod ui;
mod utils;
pub mod tasks;
mod state;
pub mod consts;
pub mod types {
    pub use super::ui::notebook::updates::Updates;
    pub type AMState = std::sync::Arc<std::sync::Mutex<super::state::State>>;
}
use std::sync::{Arc, Mutex};
use library_loader_core::{
    Config,
    is_debug
};
use ui::Ui;

fn main() {

    let state: types::AMState = Arc::new(Mutex::new(state::State {
        save_login_info: true,
        config: Config::load()
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

        if !is_debug!() {
            // Updates
            tasks::check_updates(&ui.notebook.updates);
        }

    });

    application.run(&args().collect::<Vec<_>>());

    println!("{:#?}", state);

}
