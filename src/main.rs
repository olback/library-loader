use gio::prelude::*;
use std::env::args;
mod ui;
mod utils;
pub mod consts;

fn main() {

    // Load resources
    utils::load_resources();

    // Create application
    let application =
    gtk::Application::new(Some("net.olback.library-loader"), Default::default())
    .unwrap();

    application.connect_activate(|app| {
        // Build ui
        ui::build(app);
    });

    application.run(&args().collect::<Vec<_>>());

}
