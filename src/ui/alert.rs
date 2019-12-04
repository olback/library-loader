use gtk::{Builder, Dialog, Label, Button, prelude::*};

pub struct Alert {
    dialog: Dialog,
    label: Label
}

impl Alert {

    pub fn new(builder: &Builder, name: &str) -> Self {

        let dialog: Dialog = builder.get_object(&format!("{}_dialog", name)).expect(&format!("could not find {}_dialog", name));
        let label: Label = builder.get_object(&format!("{}_label", name)).expect(&format!("could not find {}_label", name));
        let button: Button = builder.get_object(&format!("{}_close", name)).expect(&format!("could not find {}_button", name));

        let dialog_clone = dialog.clone();
        button.connect_clicked(move |_| {
            dialog_clone.hide();
        });

        dialog.hide_on_delete();

        Self {
            dialog: dialog,
            label: label
        }

    }

    pub fn set_text(&self, message: &str) {
        self.label.set_text(&message);
    }

    pub fn show(&self) {
        match self.dialog.run() {
            _ => self.hide()
        }
    }

    pub fn hide(&self) {
        self.dialog.hide()
    }

}
