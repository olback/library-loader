use {
    crate::error::{self, Error},
    std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
    },
};

pub struct Result {
    pub output_path: PathBuf,
    pub files: HashMap<String, Vec<u8>>,
}

impl Result {
    pub fn save(&self) -> error::Result<PathBuf> {
        let save_dir = Path::new(&self.output_path);

        if &self.files.len() > &0 {
            if !save_dir.exists() {
                fs::create_dir_all(save_dir)?;
            }

            for (filename, data) in &self.files {
                let path = save_dir.join(filename);
                Self::write(path, data.to_vec())?;
            }

            Ok(save_dir.canonicalize()?)
        } else {
            // Err(new_err!("No files found for your specified library"))
            Err(Error::NoFilesInLibrary)
        }
    }

    fn write(path: PathBuf, data: Vec<u8>) -> error::Result<PathBuf> {
        if path.exists() {
            // return Err(new_err!(format!("{} already exists!", p)));
            return Err(Error::WouldOverwrite);
        }

        fs::write(&path, &data)?;
        Ok(path)
    }
}
