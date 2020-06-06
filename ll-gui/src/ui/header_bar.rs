use gtk::{Builder, prelude::*};
use crate::{
    types::AMState,
    utils::safe_lock
};

#[derive(Debug)]
pub struct HeaderBar {
    header_bar: gtk::HeaderBar
}

impl HeaderBar {

    pub fn build(builder: &Builder, state: &AMState) -> Self {

        let inner = Self {
            header_bar: builder.get_object("header_bar").expect("could not find header_bar")
        };

        let path = Self::get_path(&state);
        if path.is_some() {
            inner.header_bar.set_subtitle(Some(path.unwrap().as_str()));
        } else {
            inner.header_bar.set_subtitle(None);
        }

        inner

    }

    fn get_path(state: &AMState) -> Option<String> {

        safe_lock(&state, |lock| {
            match &lock.config.config_file {
                Some(v) => Some(v.to_string_lossy().to_string()),
                None => None
            }
        })

    }

}
