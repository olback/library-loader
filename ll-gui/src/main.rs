use gio::prelude::*;
use std::env::args;
mod ui;
mod utils;
pub mod consts;

use ui::Ui;

fn main() {

    // Load resources
    utils::load_resources();

    // Create application
    let application =
    gtk::Application::new(Some("net.olback.library-loader"), Default::default())
    .unwrap();

    application.connect_activate(move |app| {

        // Build ui
        let ui = Ui::build(app);

        ui.notebook.watch.set_format("kicad");

        // ui.error_dialog.set_text("Hello this is a message.");
        // ui.error_dialog.show();

    });

    application.run(&args().collect::<Vec<_>>());

}
