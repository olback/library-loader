use crate::types::AMState;
use library_loader_core::Format;
use gtk::{
    ApplicationWindow,
    Builder,
    Button,
    ComboBoxText,
    Entry,
    FileChooserDialog,
    FileChooserAction,
    Label,
    ToggleButton,
    ResponseType,
    prelude::*
};
use crate::tasks::watcher;

#[derive(Debug, Clone)]
pub struct Watch {
    format: ComboBoxText,
    watch_folder_entry: Entry,
    watch_folder_button: Button,
    watch_folder_dialog: FileChooserDialog,
    output_folder_entry: Entry,
    output_folder_button: Button,
    output_folder_dialog: FileChooserDialog,
    start_button: ToggleButton,
    status: Label
    // TODO: glib::MainContext::channel<String>?
}

impl Watch {

    pub fn build(builder: &Builder, main_window: &ApplicationWindow, state: &AMState) -> Self {

        let inner = Self {
            format: builder.get_object("watch_format").expect("could not get watch_format"),
            watch_folder_entry: builder.get_object("watch_watch_folder").expect("could not get watch_watch_folder"),
            watch_folder_button: builder.get_object("watch_open_watch_dialog").expect("could not get watch_open_watch_dialog"),
            watch_folder_dialog: FileChooserDialog::with_buttons(Some("Watch Folder"), Some(main_window), FileChooserAction::SelectFolder, &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)]),
            output_folder_entry: builder.get_object("watch_output_folder").expect("could not get watch_output_folder"),
            output_folder_button: builder.get_object("watch_open_output_dialog").expect("could not get watch_open_output_dialog"),
            output_folder_dialog: FileChooserDialog::with_buttons(Some("Output Folder"), Some(main_window), FileChooserAction::SelectFolder, &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)]),
            start_button: builder.get_object("watch_button").expect("could not get watch_button"),
            status: builder.get_object("watch_status").expect("could not get watch_status")
        };

        // Set initial values
        let state_lock = state.lock().unwrap();
        Self::set_format(&inner.format, &state_lock.config.settings.format.name);
        let wp = match &state_lock.config.settings.watch_path {
            Some(v) => v.clone(),
            None => String::new()
        };
        inner.watch_folder_entry.set_text(&wp);
        inner.output_folder_entry.set_text(&state_lock.config.settings.output_path);
        drop(state_lock);

        // Format changed signal
        let format_state = state.clone();
        inner.format.connect_changed(move |f| {

            let res = f.get_active_id().unwrap().to_string();
            let mut format_state_lock = format_state.lock().unwrap();
            format_state_lock.config.settings.format = Format::from(&res);
            drop(format_state_lock);
            println!("{}", res);


        });

        // Watch folder dialog setup
        let watch_dialog_state = state.clone();
        let watch_dialog_clone = inner.watch_folder_dialog.clone();
        let watch_folder_entry = inner.watch_folder_entry.clone();
        inner.watch_folder_button.connect_clicked(move |_| {

            let res = watch_dialog_clone.run();
            watch_dialog_clone.hide_on_delete();

            if res == ResponseType::Accept {
                let path = watch_dialog_clone.get_filename().unwrap();
                let path_str = path.into_os_string().into_string().unwrap();
                watch_folder_entry.set_text(&path_str);
                let mut lock = watch_dialog_state.lock().unwrap();
                lock.config.settings.watch_path = Some(path_str);
                drop(lock);
            }

        });

        // Output folder dialog setup
        let output_dialog_state = state.clone();
        let output_dialog_clone = inner.watch_folder_dialog.clone();
        let out_folder_entry = inner.output_folder_entry.clone();
        inner.output_folder_button.connect_clicked(move |_| {

            let res = output_dialog_clone.run();
            output_dialog_clone.hide_on_delete();

            if res == ResponseType::Accept {
                let path = output_dialog_clone.get_filename().unwrap();
                let path_str = path.into_os_string().into_string().unwrap();
                out_folder_entry.set_text(&path_str);
                let mut lock = output_dialog_state.lock().unwrap();
                lock.config.settings.output_path = path_str;
                drop(lock);
            }

        });

        let run_state = state.clone();
        inner.start_button.connect_clicked(move |b| {

            let run_state_lock = run_state.lock().unwrap();
            println!("{:#?}", *run_state_lock);
            drop(run_state_lock);

            if b.get_active() {
                b.set_label("Stop");
                println!("Starting...");
            } else {
                b.set_label("Start");
                println!("Stopping...");
            }

        });

        inner

    }

    fn set_format(format_combo_box: &ComboBoxText, format: &str) {

        let model = format_combo_box.get_model().expect("could not get model");
        let iter = model.get_iter_from_string("0:1").expect("failed to get iter from string");
        let mut index = 0;

        loop {

            let val: String = model.get_value(&iter, 1).get().expect("failed to get value");
            println!("val: {}, index: {}", val, index);

            if val == format {
                format_combo_box.set_active(Some(index));
                break;
            }

            index += 1;

            if !model.iter_next(&iter) {
                break;
            }

            // In case something goes wrong, just crash instead of looping indefinitely
            if index > 100 {
                unreachable!("no match found")
            }

        }

    }

    pub fn set_watch_folder(&self, path: &str) {

        self.watch_folder_entry.set_text(path);

    }

    pub fn set_output_folder(&self, path: &str) {

        self.output_folder_entry.set_text(path);

    }

}
