use {
    gtk::{
        gdk::{self, prelude::*},
        gio,
        glib::{self, clone},
        prelude::*,
        CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
    },
    ll_core::{Config, Result},
    std::{cell::RefCell, rc::Rc},
    ui::Ui,
};

mod consts;
mod macros;
mod ui;

fn main() -> Result<()> {
    let config_path = match Config::get_path()? {
        Some(p) => p,
        None => {
            Config::default().save(None)?;
            Config::default_path().expect("Failed to get default path")
        }
    };

    println!("Using config: {:?}", config_path);

    let config = Rc::new(RefCell::new(Config::read(Some(config_path.clone()))?));

    // Load resources
    load_resources();

    // Create application
    let application = gtk::Application::new(Some("net.olback.library-loader"), Default::default());

    application.connect_activate(clone!(@weak config => move |app| {
        // Load CSS
        let provider = CssProvider::new();
        provider.load_from_resource(resource!("app.css"));
        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Build ui
        let u = Ui::new(app, config, config_path.clone());
        #[cfg(not(debug_assertions))]
        u.check_logged_in();
        #[cfg(not(debug_assertions))]
        u.check_updates();
    }));

    // Run app
    application.run();

    // Before exit, save config
    config.borrow().save(None)?;
    Ok(())
}

pub fn load_resources() {
    let glib_resource_bytes = glib::Bytes::from_static(consts::RESOURCES_BYTES);
    let resources =
        gio::Resource::from_data(&glib_resource_bytes).expect("Failed to load resources");
    gio::resources_register(&resources);
}
