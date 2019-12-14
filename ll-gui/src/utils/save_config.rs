use library_loader_core::{LLResult, ParseConfig, ParseSettings};
use toml;
use crate::types::State;
use std::fs;

pub fn save_config(state: &State) -> LLResult<bool> {

    if state.config.config_file.is_some() &&
    state.config.profile.username.len() > 0 &&
    state.config.profile.password.len() > 0 {

        let settings = ParseSettings {
            output_path: Some(state.config.settings.output_path.clone()),
            watch_path: state.config.settings.watch_path.clone(),
            format: Some(state.config.settings.format.name.clone())
        };

        let profile = match state.save_login_info {
            true => Some(state.config.profile.clone()),
            false => None
        };

        let fs_config = ParseConfig {
            settings: Some(settings),
            profile: profile
        };

        let toml_str = toml::to_string_pretty(&fs_config)?;
        fs::write(state.config.config_file.clone().unwrap(), toml_str)?;

        return Ok(true)

    }

    Ok(false)

}
