use gtk::{Builder, Dialog, Label, Button, prelude::*};
use crate::{
    types::AMState,
    utils::safe_lock
};

#[derive(Debug)]
pub struct Alert {
    dialog: Dialog,
    label: Label,
    button: Button
}

impl Alert {

    pub fn new(builder: &Builder, name: &str, state: &AMState) -> Self {

        let inner = Self {
            dialog: builder.get_object(&format!("{}_dialog", name)).expect(&format!("could not find {}_dialog", name)),
            label: builder.get_object(&format!("{}_label", name)).expect(&format!("could not find {}_label", name)),
            button: builder.get_object(&format!("{}_close", name)).expect(&format!("could not find {}_button", name))
        };

        inner.dialog.hide_on_delete();

        let dialog_clone = inner.dialog.clone();
        inner.button.connect_clicked(move |_| {
            dialog_clone.hide();
        });

        let run_dialog_clone = inner.dialog.clone();
        let run_dialog_label_clone = inner.label.clone();
        safe_lock(&state, move |lock| {
            match name {
                "warning" => {
                    match lock.get_warning_rx() {
                        Some(rx) => {
                            rx.attach(None, move |message| {
                                run_dialog_label_clone.set_text(&message);
                                match run_dialog_clone.run() {
                                    _ => run_dialog_clone.hide()
                                }
                                glib::Continue(true)
                            });
                        },
                        None => {}
                    }
                },
                "error" => {
                    match lock.get_error_rx() {
                        Some(rx) => {
                            rx.attach(None, move |message| {
                                run_dialog_label_clone.set_text(&message);
                                match run_dialog_clone.run() {
                                    _ => run_dialog_clone.hide()
                                }
                                glib::Continue(true)
                            });
                        },
                        None => {}
                    }
                },
                _ => {}
            }
        });

        inner

    }

}
