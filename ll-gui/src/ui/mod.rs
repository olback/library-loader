use {
    crate::{get_obj, resource},
    event::UiEvent,
    gtk::{
        gio::File,
        glib::{self, clone},
        prelude::*,
        AboutDialog, Align, Application, ApplicationWindow, Box as GtkBox, Builder, Button,
        ComboBoxText, Dialog, Entry, FileChooserButton, HeaderBar, IconSize, InfoBar, Label,
        ListBox, MessageType, Orientation, ResponseType, Stack, Switch, TextBuffer,
    },
    ll_core::{Config, Format, Watcher, ECAD},
    logger::GuiLogger,
    std::{cell::RefCell, path::PathBuf, rc::Rc, thread},
};

mod event;
mod logger;

pub struct Ui {
    config: Rc<RefCell<Config>>,
    info_bar: InfoBar,
    info_bar_label: Label,
    main_stack: Stack,
    username_entry: Entry,
    password_entry: Entry,
    login_button: Button,
    add_format_button: Button,
    add_format_dialog: Dialog,
    add_format_name: Entry,
    add_format_format: ComboBoxText,
    add_format_output: FileChooserButton,
    formats_list_box: ListBox,
    info_bar_update: InfoBar,
    tx: glib::Sender<UiEvent>,
    watcher: RefCell<Option<Watcher>>,
}

impl Ui {
    pub fn new(app: &Application, config: Rc<RefCell<Config>>, config_path: PathBuf) -> Rc<Self> {
        // Builder
        let b = Builder::from_resource(resource!("ui"));

        // Connect main window to app
        get_obj!(b, ApplicationWindow, "main-window").set_application(Some(app));
        get_obj!(b, HeaderBar, "header-bar").set_subtitle(config_path.to_str());

        // Configure about dialog
        let about_dialog = get_obj!(b, AboutDialog, "about-dialog");
        get_obj!(b, Button, "open-about-dialog").connect_clicked(move |_btn| {
            about_dialog.run();
            about_dialog.hide();
        });

        // Event channel
        let (tx, rx) = glib::MainContext::channel::<UiEvent>(glib::Priority::default());

        // Things to keep
        let inner = Rc::new(Self {
            config,
            info_bar: get_obj!(b, "info-bar"),
            info_bar_label: get_obj!(b, "info-bar-label"),
            main_stack: get_obj!(b, "main-stack"),
            username_entry: get_obj!(b, "username-entry"),
            password_entry: get_obj!(b, "password-entry"),
            login_button: get_obj!(b, "login-button"),
            add_format_button: get_obj!(b, "add-format-button"),
            add_format_dialog: get_obj!(b, "add-format-dialog"),
            add_format_name: get_obj!(b, "add-format-name"),
            add_format_format: get_obj!(b, "add-format-format"),
            add_format_output: get_obj!(b, "add-format-output"),
            formats_list_box: get_obj!(b, "formats-list-box"),
            info_bar_update: get_obj!(b, "info-bar-update"),
            tx,
            watcher: RefCell::new(None),
        });

        // Info bar close events
        inner.info_bar.connect_response(|ib, _| {
            ib.set_revealed(false);
        });

        inner.info_bar_update.connect_response(|ib, _| {
            ib.set_revealed(false);
        });

        // Listen on event channel
        rx.attach(
            None,
            clone!(@strong inner => move |event| {
                // println!("Event: {:#?}", event);
                match event {
                    UiEvent::ShowInfoBar(msg, msg_type) => {
                        inner.info_bar_label.set_text(&msg);
                        inner.info_bar.set_message_type(msg_type);
                        inner.info_bar.set_revealed(true);
                    },
                    UiEvent::SwitchStack(name) => {
                        inner.main_stack.set_visible_child_name(name)
                    },
                    UiEvent::SetProfile(profile) => {
                        inner.config.borrow_mut().profile = profile;
                    }
                    UiEvent::UpdateAvailable => {
                        inner.info_bar_update.set_revealed(true);
                    },
                    UiEvent::UpdateFormats => {
                        for w in &inner.formats_list_box.children() {
                            inner.formats_list_box.remove(w);
                        }
                        for (name, format) in &inner.config.borrow().formats {
                            let box1 = GtkBox::new(Orientation::Horizontal, 2);
                            box1.set_spacing(6);
                            box1.set_border_width(6);
                            let box2 = GtkBox::new(Orientation::Vertical, 3);
                            box2.set_spacing(6);
                            box2.set_hexpand(true);
                            box2.set_hexpand_set(true);
                            let label1 = Label::new(Some(name));
                            label1.set_halign(Align::Start);
                            label1.set_xalign(0.0);
                            label1.style_context().add_class("format-title");
                            let label2 = Label::new(Some(&format!("Format: {}", format.format)));
                            label2.set_halign(Align::Start);
                            label2.set_xalign(0.0);
                            label2.style_context().add_class("dim-label");
                            let label3 = Label::new(Some(&format!("Output path: {}", format.output_path)));
                            label3.set_halign(Align::Start);
                            label3.set_xalign(0.0);
                            label3.style_context().add_class("dim-label");
                            box2.add(&label1);
                            box2.add(&label2);
                            box2.add(&label3);
                            let button = Button::from_icon_name(Some("edit-delete"), IconSize::Button);
                            let name = name.clone();
                            button.connect_clicked(clone!(@strong inner => move |_btn| {
                                inner.config.borrow_mut().formats.remove(&name);
                                drop(inner.tx.send(UiEvent::UpdateFormats));
                            }));
                            button.set_halign(Align::Center);
                            button.set_valign(Align::Center);
                            box1.add(&box2);
                            box1.add(&button);
                            box1.show_all();
                            inner.formats_list_box.add(&box1);
                        }
                    }
                }
                glib::ControlFlow::Continue
            }),
        );

        // Login
        inner
            .login_button
            .connect_clicked(clone!(@strong inner => move |_btn| {
                let tx = inner.tx.clone();
                drop(tx.send(UiEvent::SwitchStack("spinner")));
                let profile = ll_core::Profile {
                    username: inner.username_entry.text().to_string(),
                    password: inner.password_entry.text().to_string(),
                };
                thread::spawn(move || {
                    match profile.try_auth() {
                        Ok(true) => {
                            drop(tx.send(UiEvent::SetProfile(profile)));
                            drop(tx.send(UiEvent::SwitchStack("watch")));
                        },
                        Ok(false) => {
                            drop(tx.send(UiEvent::SwitchStack("login")));
                            drop(tx.send(UiEvent::ShowInfoBar("Login failed".into(), MessageType::Error)))
                        },
                        Err(e) => {
                            drop(tx.send(UiEvent::SwitchStack("login")));
                            drop(tx.send(UiEvent::ShowInfoBar(format!("Login failed: {:?}", e), MessageType::Error)))
                        }
                    }
                });
            }));

        // Watch path
        let watch_path_chooser = get_obj!(b, FileChooserButton, "watch-path");
        let _ = watch_path_chooser.set_current_folder_file(&File::for_path(PathBuf::from(
            shellexpand::full(&inner.config.borrow().settings.watch_path)
                .unwrap()
                .as_ref(),
        )));
        watch_path_chooser.connect_file_set(clone!(@strong inner => move |btn| {
            if let Some(p) = btn.file().and_then(|f| f.path()) {
                inner.config.borrow_mut().settings.watch_path = p.to_str().unwrap().to_string();
            }
        }));
        let recursive_switch = get_obj!(b, Switch, "watch-recursive");
        recursive_switch.set_active(inner.config.borrow().settings.recursive);
        recursive_switch.connect_state_notify(clone!(@strong inner => move |switch| {
            inner.config.borrow_mut().settings.recursive = switch.is_active();
        }));

        // Add format
        inner
            .add_format_dialog
            .add_buttons(&[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);
        inner
            .add_format_button
            .connect_clicked(clone!(@strong inner => move |_btn| {
                if inner.add_format_dialog.run() == ResponseType::Ok {
                    let name = inner.add_format_name.text().to_string().trim().to_string();
                    let format = inner.add_format_format.active_id().map(|f| f.to_string());
                    let file = inner.add_format_output.file().and_then(|f| f.path());
                    if let (false, Some(format), Some(file)) = (name.is_empty(), format, file) {
                        let mut conf = inner.config.borrow_mut();
                        if conf.formats.get(&name).is_none() {
                            use std::convert::TryFrom;
                            conf.formats.insert(name, Format {
                                format: ECAD::try_from(format.as_str()).expect("Invalid ECAD type in glade file"),
                                output_path: file.to_str().unwrap().to_string()
                            });
                            drop(inner.tx.send(UiEvent::UpdateFormats));
                        } else {
                            drop(inner.tx.send(UiEvent::ShowInfoBar(format!("Format with name '{}' already exists", name), MessageType::Error)));
                        }
                    }
                }
                inner.add_format_dialog.hide();
            }));

        // Logger
        let text_buffer = get_obj!(b, TextBuffer, "output-log-buffer");
        let mut text_buffer_bounds = text_buffer.bounds();
        text_buffer.delete(&mut text_buffer_bounds.0, &mut text_buffer_bounds.1);
        let (logger_rx, logger) = GuiLogger::new();
        logger_rx.attach(
            None,
            clone!(@strong inner => move |msg| {
                text_buffer.insert(&mut text_buffer.end_iter(), &format!("{}\n", msg));
                glib::ControlFlow::Continue
            }),
        );

        // Start button
        let watch_path_label = get_obj!(b, Label, "watch-path-label");
        get_obj!(b, Button, "start-watch-button").connect_clicked(
            clone!(@strong inner => move |_| {
                watch_path_label.set_text(&inner.config.borrow().settings.watch_path);
                match Watcher::new(inner.config.borrow().clone(), vec![ll_core::ConsoleLogger::new(), logger.clone()]).and_then(|mut w| { w.start()?; Ok(w) }) {
                    Ok(w) => {
                        *inner.watcher.borrow_mut() = Some(w);
                        drop(inner.tx.send(UiEvent::SwitchStack("log")));
                    },
                    Err(e) => drop(inner.tx.send(UiEvent::ShowInfoBar(format!("Error: {:?}", e), MessageType::Error)))
                }
            }),
        );

        // Stop button
        get_obj!(b, Button, "stop-watch-button").connect_clicked(
            clone!(@strong inner => move |_| {
                if let Some(mut w) = inner.watcher.borrow_mut().take() {
                    w.stop();
                    drop(inner.tx.send(UiEvent::SwitchStack("watch")));
                }
            }),
        );

        // temporary
        inner.main_stack.set_visible_child_name("watch");

        // Update formats list
        drop(inner.tx.send(UiEvent::UpdateFormats));

        inner
    }

    pub fn check_logged_in(&self) {
        if self.config.borrow().profile.is_empty() {
            let _ = self.tx.send(UiEvent::SwitchStack("login"));
        } else {
            let tx = self.tx.clone();
            let profile = self.config.borrow().profile.clone();
            thread::spawn(move || match profile.try_auth() {
                Ok(true) => drop(tx.send(UiEvent::SwitchStack("watch"))),
                Ok(false) => drop(tx.send(UiEvent::SwitchStack("login"))),
                Err(e) => {
                    eprintln!("{:#?}", e)
                }
            });
        }
    }

    pub fn check_updates(&self) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            match ll_core::check_updates(env!("CARGO_PKG_VERSION"), ll_core::ClientKind::GUI) {
                Ok(None) => {} // No update availabel
                Ok(Some(_)) => drop(tx.send(UiEvent::UpdateAvailable)),
                Err(e) => {
                    eprintln!("{:#?}", e)
                }
            }
        });
    }
}
