use gtk::{Builder, Button, Window, prelude::*};

pub struct Devtools {
    window: Window,
    pub button1: Button,
    pub button2: Button,
    pub button3: Button
}

impl Devtools {

    pub fn build(builder: &Builder) -> Self {

        let window: Window =  builder.get_object("devtools_window").expect("could not get devtools_window");
        window.hide_on_delete();

        Self {
            window: window,
            button1: builder.get_object("devbutton1").expect("could not get button1"),
            button2: builder.get_object("devbutton2").expect("could not get button2"),
            button3: builder.get_object("devbutton3").expect("could not get button3")
        }

    }

    pub fn show(&self) {
        self.window.show();
    }

}
