use gtk;
pub(super) use super::consts;

mod main;
mod about;

pub fn build(app: &gtk::Application) {

    let builder = gtk::Builder::new_from_string(consts::GLADE_STRING);

    main::build(&builder, &app);
    about::build(&builder);

}
