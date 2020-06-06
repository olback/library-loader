use gtk::{
    prelude::*,
    AboutDialog,
    Builder,
    Button
};

pub fn build(builder: &Builder) -> AboutDialog {

    let open_about_dialog_button: Button = builder.get_object("open_about_dialog").expect("Could not find open_about_dialog");
    let about_dialog: AboutDialog = builder.get_object("about_dialog").expect("Could not find about_dialog");

    let about_dialog_clone = about_dialog.clone();
    open_about_dialog_button.connect_clicked(move |_| {
        let res = about_dialog_clone.run();
        match res {
            _ => about_dialog_clone.hide()
        };
    });

    about_dialog.hide_on_delete();

    about_dialog.set_version(Some(env!("CARGO_PKG_VERSION")));

    about_dialog

}
