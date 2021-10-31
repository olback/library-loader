use gtk;
pub(super) use super::{consts, types::AMState};

pub mod main;
pub mod about;
pub mod alert;
pub mod notebook;
pub mod header_bar;

#[derive(Debug)]
pub struct Ui {
    pub main: gtk::ApplicationWindow,
    pub header_bar: header_bar::HeaderBar,
    pub notebook: notebook::Notebook,
    pub about_dialog: gtk::AboutDialog,
    pub warning_dialog: alert::Alert,
    pub error_dialog: alert::Alert
}

impl Ui {

    pub fn build(app: &gtk::Application, state: &AMState) -> Self {

        let builder = gtk::Builder::from_string(consts::GLADE_STRING);

        let main = main::build(&builder, &app);
        let notebook = notebook::Notebook::build(&builder, &main, &state);

        Self {
            main: main,
            header_bar: header_bar::HeaderBar::build(&builder, &state),
            notebook: notebook,
            about_dialog: about::build(&builder),
            warning_dialog: alert::Alert::new(&builder, "warning", &state),
            error_dialog: alert::Alert::new(&builder, "error", &state)
        }

    }

}
