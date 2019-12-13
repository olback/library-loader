use gtk::{Builder, Button, Switch, Entry, Spinner, Label, prelude::*};
use crate::{types::AMState, tasks};

#[derive(Debug, Clone)]
pub struct Account {
    email: Entry,
    password: Entry,
    spinner: Spinner,
    status: Label,
    save_info: Switch
}

impl Account {

    pub fn build(builder: &Builder, state: &AMState) -> Self {

        let inner = Self {
            email: builder.get_object("login_email").expect("could not get login_email"),
            password: builder.get_object("login_password").expect("could not get login_password"),
            spinner: builder.get_object("login_spinner").expect("could not get login_spinner"),
            status: builder.get_object("login_status").expect("could not get login_status"),
            save_info: builder.get_object("login_save_info").expect("could not get login_save_info")
        };

        let state_lock = state.lock().unwrap();
        if state_lock.config.profile.username.len() > 0 {
            inner.email.set_text(&state_lock.config.profile.username);
        }
        drop(state_lock);

        let toggle_btn: Button = builder.get_object("login_password_toggle_visibility").expect("could not get password_toggle_visibility");
        let password_clone = inner.password.clone();
        toggle_btn.connect_clicked(move |_| {
            let visibility = password_clone.get_visibility();
            password_clone.set_visibility(!visibility);

        });

        let login_btn: Button = builder.get_object("login_button").expect("could not get login_button");
        let save_info_switch = inner.save_info.clone();
        let email_clone = inner.email.clone();
        let password_clone = inner.password.clone();
        let label_clone = inner.status.clone();
        let spinner_clone = inner.spinner.clone();
        let state_clone = state.clone();
        login_btn.connect_clicked(move |b| {

            let email = match email_clone.get_text() {
                Some(v) => v.to_string(),
                None => String::new()
            };

            let password = match password_clone.get_text() {
                Some(v) => v.to_string(),
                None => String::new()
            };

            let save_info = save_info_switch.get_state();

            tasks::login(&state_clone, save_info, email, password, &label_clone, &spinner_clone, b);

        });

        inner

    }

    pub fn get_email(&self) -> String {

        match self.email.get_text() {
            Some(v) => String::from(v),
            None => String::new()
        }

    }

    pub fn get_password(&self) -> String {

        match self.password.get_text() {
            Some(v) => String::from(v),
            None => String::new()
        }

    }

    pub fn clear_password(&self) {

        self.password.set_text("");

    }

    pub fn show_spinner(&self, show: bool) {

        if show {
            self.spinner.set_visible(true);
            self.spinner.start();
        } else {
            self.spinner.set_visible(false);
            self.spinner.stop();
        }

    }

    pub fn show_status(&self, message: &str) {

        self.status.set_visible(true);
        self.status.set_text(message);

    }

    pub fn hide_status(&self) {

        self.status.set_visible(false);
        self.status.set_text("");

    }

}

