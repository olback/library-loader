use super::error::{LLResult, LLError};
use std::{path::Path, fs};
use super::config::Config;

pub struct CSEResult {
    pub filename: String,
    pub data: Vec<u8>
}

impl CSEResult {

    pub fn save(&self, config: &Config) -> LLResult<String> {

        let save_dir = Path::new(&config.settings.output_path);
        let path = save_dir.join(&self.filename);

        if !save_dir.exists() {
            fs::create_dir_all(save_dir)?;
        }

        match fs::write(&path, &self.data) {
            Ok(_) => Ok(path.to_str().unwrap().to_string()),
            Err(e) => Err(LLError::new(format!("{}", e)))
        }

    }

}
