use gtk::{Builder, TextBuffer, TextView, prelude::*};
use crate::{
    types::AMState,
    utils::safe_lock
};

#[derive(Debug, Clone)]
pub struct Output {
    buffer: TextBuffer,
    text_view: TextView
}

impl Output {

    pub fn build(builder: &Builder, state: &AMState) -> Self {

        let inner = Self {
            buffer: builder.get_object("output_log_buffer").expect("could not get output_log_buffer"),
            text_view: builder.get_object("output_log").expect("could not get output_log")
        };

        inner.buffer.set_text("");

        safe_lock(&state.clone(), |lock| {

            let buffer_clone = (&inner.buffer).clone();
            let text_view_clone = (&inner.text_view).clone();

            match lock.get_log_rx() {
                Some(rx) => Self::attach_listener(rx, buffer_clone, text_view_clone),
                None => eprintln!("Logger already configured!")
            };

        });

        inner

    }

    fn append(buffer: &TextBuffer, text: &str) {
        let lines = buffer.get_line_count();
        let mut iter = buffer.get_iter_at_line(lines);

        buffer.insert(&mut iter, text);
    }

    fn attach_listener(rx: glib::Receiver<String>, buffer: TextBuffer, text_view: TextView) {

        rx.attach(None, move |message| {

            println!("{}", message);

            // Append message to buffer
            Self::append(&buffer, &message);

            // New line
            #[cfg(not(windows))]
            Self::append(&buffer, "\n");

            #[cfg(windows)]
            Self::append(&buffer, "\r\n");

            // Scroll to the bottom
            let lines = *&buffer.get_line_count();
            let mut iter = buffer.get_iter_at_line(lines);
            &text_view.scroll_to_iter(&mut iter, 0f64, false, 0f64, 0f64);

            // Continue
            glib::Continue(true)

        });

    }

}
