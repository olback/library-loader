use library_loader_core::{Profile, LLResult, ParseConfig, ParseSettings};
use toml;
use crate::types::State;
use std::fs;

pub fn save_config(state: &State) -> LLResult<bool> {

    if state.config.config_file.is_some() {

        let settings = ParseSettings {
            output_path: Some(state.config.settings.output_path.clone()),
            watch_path: state.config.settings.watch_path.clone(),
            format: Some(state.config.settings.format.name.clone())
        };

        let profile = match state.save_login_info {
            true => Some(state.config.profile.clone()),
            false => Some(Profile::new(state.config.profile.username.clone(), ""))
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
