use gtk;
pub(super) use super::consts;

mod main;
mod about;
mod alert;
mod notebook;

pub struct Ui {
    pub main: gtk::ApplicationWindow,
    pub notebook: notebook::Notebook,
    pub about_dialog: gtk::AboutDialog,
    pub warning_dialog: alert::Alert,
    pub error_dialog: alert::Alert
}

impl Ui {

    pub fn build(app: &gtk::Application) -> Self {

        let builder = gtk::Builder::new_from_string(consts::GLADE_STRING);

        Self {
            main: main::build(&builder, &app),
            notebook: notebook::Notebook::build(&builder),
            about_dialog: about::build(&builder),
            warning_dialog: alert::Alert::new(&builder, "warning"),
            error_dialog: alert::Alert::new(&builder, "error")
        }

    }

}
