use gtk::{Builder, Image, Label, prelude::*};

#[derive(Debug, Clone)]
pub struct Updates {
    icon: Image,
    label: Label,
    tab_label: Label
}

impl Updates {

    pub fn build(builder: &Builder) -> Self {

        Self {
            icon: builder.get_object("updates_icon").expect("could not get updates_icon"),
            label: builder.get_object("updates_label").expect("could not get updates_label"),
            tab_label: builder.get_object("notebook_updates_label").expect("could not get notebook_updates_label"),
        }

    }

    pub fn set_available(&self, new_version: &str) {
        self.icon.set_from_resource(Some("/net/olback/library-loader/icon/download"));
        self.label.set_text(&format!("New update available!\nVersion {} is now available.", new_version));
        self.tab_label.set_text("Updates*");
    }

    pub fn set_error(&self) {
        self.icon.set_from_resource(Some("/net/olback/library-loader/icon/exclamation"));
        self.label.set_text("Error checking for updates");
    }

}