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
}

impl Watch {

    pub fn build(builder: &Builder, main_window: &ApplicationWindow) -> Self {

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

        inner.watch_folder_dialog.hide_on_delete();
        inner.output_folder_dialog.hide_on_delete();

        let watch_dialog_clone = inner.watch_folder_dialog.clone();
        inner.watch_folder_button.connect_clicked(move |_| {

            let res = watch_dialog_clone.run();
            watch_dialog_clone.hide_on_delete();

            if res == ResponseType::Accept {
                let path = watch_dialog_clone.get_filename();
                println!("{:#?}", path);
            }

        });

        inner.start_button.connect_clicked(move |b| {

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

    pub fn set_format(&self, format: &str) {

        let model = self.format.get_model().expect("could not get model");
        let iter = model.get_iter_from_string("0:1").expect("failed to get iter from string");
        let mut index = 0;

        loop {

            let val: String = model.get_value(&iter, 1).get().expect("failed to get value");
            println!("val: {}, index: {}", val, index);

            if val == format {
                self.format.set_active(Some(index));
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
