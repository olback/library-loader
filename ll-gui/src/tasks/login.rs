use crate::types::AMState;
use gtk::{Button, Label, Spinner, prelude::*};
use library_loader_core::Profile;

enum LoginStatus {
    Success(Profile),
    Error(String)
}

pub fn login(state: &AMState, save_info: bool, email: String, password: String, status: &Label, spinner: &Spinner, button: &Button) -> std::thread::JoinHandle<()> {

    let local_state = state.clone();
    let local_status = status.clone();
    let local_spinner = spinner.clone();
    let local_button = button.clone();

    local_status.set_text("");
    local_spinner.start();
    local_button.set_sensitive(false);


    let (tx, rx) = glib::MainContext::channel::<LoginStatus>(glib::PRIORITY_DEFAULT);

    rx.attach(None, move |status| {

        local_spinner.stop();
        local_button.set_sensitive(true);

        match status {
            LoginStatus::Success(profile) => {
                local_status.set_text("Login successful");
                let mut state_lock = local_state.lock().unwrap();
                state_lock.save_login_info = save_info;
                state_lock.config.profile = profile;
                state_lock.logged_in = true;
                drop(state_lock);
                local_button.set_label("Log out");
            },
            LoginStatus::Error(reason) => local_status.set_text(&reason)
        };

        glib::Continue(false)

    });

    return std::thread::spawn(move || {

        let profile = Profile::new(&email, &password);

        match profile.try_auth() {
            Ok(res) => {
                if res {
                    tx.send(LoginStatus::Success(profile)).unwrap();
                } else {
                    tx.send(LoginStatus::Error(String::from("Invalid credentials"))).unwrap();
                }
            },
            Err(e) => tx.send(LoginStatus::Error(format!("{}", e))).unwrap()
        }

    });

}
