use gtk::{
    prelude::*,
    AboutDialog,
    Builder,
    Button
};
use toml;
use super::consts;
use serde::Deserialize;

#[derive(Deserialize)]
struct CargoTomlPackage {
    version: String
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage
}

pub fn build(builder: &Builder) -> AboutDialog {

    let open_about_dialog_button: Button = builder.get_object("open_about_dialog").expect("Could not find open_about_dialog");
    let about_dialog: AboutDialog = builder.get_object("about_dialog").expect("Could not find about_dialog");

    let about_dialog_clone = about_dialog.clone();
    open_about_dialog_button.connect_clicked(move |_| {
        about_dialog_clone.show();
    });

    about_dialog.connect_delete_event(|ad, _| {
        ad.hide();
        Inhibit(true)
    });

    // Fetch version from Cargo.toml and show it in the About window.
    let ct: CargoToml = toml::from_str(consts::CARGO_TOML).unwrap();
    about_dialog.set_version(Some(&ct.package.version));

    about_dialog

}
