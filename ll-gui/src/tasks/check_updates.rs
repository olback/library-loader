use library_loader_core::{
    check_updates,
    UpdateInfo,
    new_err
};
use super::super::consts;
use super::super::types::Updates;


pub fn check(updates_tab: &Updates) -> std::thread::JoinHandle<()> {

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let updates_clone = updates_tab.clone();

    rx.attach(None, move |update: Option<UpdateInfo>| {

        if update.is_some() {
            updates_clone.set_available(&update.unwrap().remote);
        } else {
            updates_clone.set_error();
        }

        glib::Continue(false)

    });

    return std::thread::spawn(move || {

        match check_updates::check(env!("CARGO_PKG_VERSION"), consts::REMOTE_CARGO_TOML) {
            Ok(res) => {
                match res {
                    Some(update) => { tx.send(Some(update)).unwrap(); },
                    None => { } // No update available, do nothing.
                }
            },
            Err(e) => {
                eprintln!("{}", new_err!(e));
                tx.send(None).unwrap();
            }
        }

    });

}
