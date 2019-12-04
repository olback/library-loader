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

    application.connect_activate(|app| {
        // Build ui
        let ui = Ui::build(app);


        let output_clone1 = ui.notebook.output.clone();
        let output_clone2 = ui.notebook.output.clone();
        // Dev...
        #[cfg(debug_assertions)]
        {
            use gtk::prelude::*;
            use std::time::{SystemTime, UNIX_EPOCH};

            ui.devtools.show();

            ui.devtools.button1.connect_clicked(move |_| {
                let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
                output_clone1.append_line(&format!("{} Hello!", nanos));
                output_clone1.scroll_to_bottom();
            });

            ui.devtools.button2.connect_clicked(move |_| {
                output_clone2.clear();
            });

        }

    });

    application.run(&args().collect::<Vec<_>>());

}
