use gtk;
pub(super) use super::{consts, types::AMState};

pub mod main;
pub mod about;
pub mod alert;
pub mod notebook;

#[derive(Debug)]
pub struct Ui {
    pub main: gtk::ApplicationWindow,
    pub notebook: notebook::Notebook,
    pub about_dialog: gtk::AboutDialog,
    pub warning_dialog: alert::Alert,
    pub error_dialog: alert::Alert
}

impl Ui {

    pub fn build(app: &gtk::Application, state: &AMState) -> Self {

        let builder = gtk::Builder::new_from_string(consts::GLADE_STRING);

        let main = main::build(&builder, &app);
        let notebook = notebook::Notebook::build(&builder, &main, &state);

        Self {
            main: main,
            notebook: notebook,
            about_dialog: about::build(&builder),
            warning_dialog: alert::Alert::new(&builder, "warning"),
            error_dialog: alert::Alert::new(&builder, "error")
        }

    }

}
