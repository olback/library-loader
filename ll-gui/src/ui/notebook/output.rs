use gtk::{Builder, TextBuffer, TextView, prelude::*};

#[derive(Debug, Clone)]
pub struct Output {
    buffer: TextBuffer,
    text_view: TextView
}

impl Output {

    pub fn build(builder: &Builder) -> Self {

        Self {
            buffer: builder.get_object("output_log_buffer").expect("could not get output_log_buffer"),
            text_view: builder.get_object("output_log").expect("could not get output_log")
        }

    }

    pub fn append(&self, text: &str) {

        let lines = self.buffer.get_line_count();
        let mut iter = self.buffer.get_iter_at_line(lines);

        self.buffer.insert(&mut iter, text);

    }

    pub fn append_line(&self, text: &str) {

        self.append(text);

        #[cfg(not(windows))]
        self.append("\n");

        #[cfg(windows)]
        self.append("\r\n");

    }

    pub fn clear(&self) {

        self.buffer.set_text("");

    }

    pub fn scroll_to_bottom(&self) {

        let lines = self.buffer.get_line_count();
        let mut iter = self.buffer.get_iter_at_line(lines);

        self.text_view.scroll_to_iter(&mut iter, 0f64, false, 0f64, 0f64);

    }

}
