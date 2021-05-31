use super::{
    new_err,
    error::LLResult
};
use std::{
    path::{Path, PathBuf},
    fs,
    collections::HashMap
};

// pub struct CSEResult {
//     pub format: Format,
//     pub output_path: String,
//     pub filename: String,
//     pub data: Vec<u8>
// }

pub struct CSEUnprocessedResult
{
    pub output_path: String,
    pub files: HashMap<String, Vec<u8>>
}

pub struct CSEResult {
    pub output_path: String,
    pub files: HashMap<String, Vec<u8>>
}

impl CSEResult {

    pub fn save(&self) -> LLResult<String> {

        let save_dir = Path::new(&self.output_path);

        if &self.files.len() > &0 {

            for (filename, data) in &self.files {
                let path = save_dir.join(filename);
                let dir = path.parent().unwrap();

                if !dir.exists()
                {
                    fs::create_dir_all(dir)?;
                }

                Self::write(path, data.to_vec())?;
            }

            Ok(save_dir.canonicalize()?.to_str().unwrap_or("").to_string())

        } else {

            Err(new_err!("No files found for your specified library"))

        }

    }

    fn write(path: PathBuf, data: Vec<u8>) -> LLResult<String> {

        let p = path.to_str().unwrap().to_string();

        match fs::write(&path, &data) {
            Ok(_) => Ok(p),
            Err(e) => Err(new_err!(e))
        }

    }

}
