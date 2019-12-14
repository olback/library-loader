use gtk::{Builder, Button, CheckButton, Entry, Spinner, Label, prelude::*};
use crate::{types::AMState, tasks};
use library_loader_core::Profile;

#[derive(Debug, Clone)]
pub struct Account {
    email: Entry,
    password: Entry,
    spinner: Spinner,
    status: Label,
    save_info: CheckButton
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

        let state_set_clone = state.clone();
        inner.save_info.connect_toggled(move |cb| {
            let mut state_set_lock = state_set_clone.lock().unwrap();
            state_set_lock.save_login_info = cb.get_active();
            drop(state_set_lock);
        });

        let login_btn: Button = builder.get_object("login_button").expect("could not get login_button");

        let state_lock = state.lock().unwrap();
        inner.save_info.set_active(state_lock.save_login_info);
        if state_lock.logged_in {
            inner.email.set_text(&state_lock.config.profile.username);
            login_btn.set_label("Log out");
        }
        drop(state_lock);

        let toggle_btn: Button = builder.get_object("login_password_toggle_visibility").expect("could not get password_toggle_visibility");
        let password_clone = inner.password.clone();
        toggle_btn.connect_clicked(move |_| {
            let visibility = password_clone.get_visibility();
            password_clone.set_visibility(!visibility);

        });

        let save_info_switch = inner.save_info.clone();
        let email_clone = inner.email.clone();
        let password_clone = inner.password.clone();
        let label_clone = inner.status.clone();
        let spinner_clone = inner.spinner.clone();
        let state_clone = state.clone();
        login_btn.connect_clicked(move |b| {

            let state_lock = state_clone.lock().unwrap();
            let logged_in = state_lock.logged_in;
            drop(state_lock);

            if logged_in {

                // Log out
                b.set_label("Login");
                label_clone.set_text("Log out successful");
                let mut state_lock = state_clone.lock().unwrap();
                state_lock.logged_in = false;
                state_lock.config.profile = Profile::new("", "");
                drop(state_lock);

            } else {

                let email = match email_clone.get_text() {
                    Some(v) => v.to_string(),
                    None => String::new()
                };

                let password = match password_clone.get_text() {
                    Some(v) => v.to_string(),
                    None => String::new()
                };

                let save_info = save_info_switch.get_active();

                tasks::login(&state_clone, save_info, email, password, &label_clone, &spinner_clone, b);

            }

        });

        inner

    }

}

