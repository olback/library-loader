use crate::{
    types::AMState,
    utils::safe_lock
};
use gtk::{Builder, TextBuffer, prelude::*};
use library_loader_core::{new_err, ParseConfig, ParseSettings, LLResult};
use toml;

#[derive(Debug, Clone)]
pub struct Configuration {
    buffer: TextBuffer
}

impl Configuration {

    pub fn build(builder: &Builder, state: &AMState) -> Self {

        let inner = Self {
            buffer: builder.get_object("configuration_buffer").expect("could not get configuration_buffer")
        };

        let logger = safe_lock(&state, |lock| {
            lock.get_log_tx()
        });

        match Self::state_to_toml(&state) {
            Ok(s) => inner.buffer.set_text(&s),
            Err(e) => logger.send(format!("{}", e)).unwrap()
        };

        inner

    }

    fn state_to_toml(state: &AMState) -> LLResult<String> {

        let conf: ParseConfig = safe_lock(&state, |lock| {

            let settings = ParseSettings {
                output_path: Some(lock.config.settings.output_path.clone()),
                watch_path: lock.config.settings.watch_path.clone(),
                format: Some(lock.config.settings.format.name.clone())
            };

            let mut profile = lock.config.profile.clone();
            profile.password = String::from("<redacted>");

            ParseConfig {
                settings: Some(settings),
                profile: Some(profile)
            }

        });

        match toml::to_string_pretty(&conf) {
            Ok(v) => Ok(v),
            Err(e) => Err(new_err!(e))
        }

    }

}
