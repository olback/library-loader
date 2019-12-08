use gtk::{
    prelude::*,
    Application,
    ApplicationWindow,
    Builder
};

pub fn build(builder: &Builder, app: &Application) -> ApplicationWindow {

    let window: ApplicationWindow = builder.get_object("main_window").expect("Could not find main_window");
    window.set_application(Some(app));
    window.show_all();

    window

}
