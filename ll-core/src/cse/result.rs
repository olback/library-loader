use {
    crate::error::{self, Error},
    std::{
        collections::HashMap,
        fs::{self, File},
        io::{BufRead, BufReader, BufWriter, Write},
        path::{Path, PathBuf},
        str,
    },
};

static FOOTPRINT_FOLDER: &str = "LibraryLoader.pretty";
static SYM_FILE: &str = "LibraryLoader.kicad_sym";

pub struct Result {
    pub output_path: PathBuf,
    pub files: HashMap<String, Vec<u8>>,
}

impl Result {
    pub fn save(&self) -> error::Result<PathBuf> {
        let save_dir = Path::new(&self.output_path);
        let parent_dir = Path::new(save_dir.parent().unwrap());
        let collected_footprints = parent_dir.join(FOOTPRINT_FOLDER);
        let collected_symbol = parent_dir.join(SYM_FILE);
        if &self.files.len() > &0 {
            if !save_dir.exists() {
                fs::create_dir_all(save_dir)?;
            }
            if !collected_footprints.exists() {
                fs::create_dir_all(collected_footprints.clone())?;
            }
            if !collected_symbol.exists() {
                {
                    let data = "(kicad_symbol_lib (version ) (generator LibraryLoader)\n)";
                    fs::write(parent_dir.join(SYM_FILE), data).expect("Unable to write file");
                }
            }
            for (filename, data) in &self.files {
                if filename.contains(".kicad_sym") {
                    let tmp_file = parent_dir.join(SYM_FILE.to_owned() + ".temp");
                    {
                        let file: File = File::open(collected_symbol.clone()).unwrap();
                        let out_file: File = File::create(tmp_file.clone()).unwrap();
                        let reader = BufReader::new(&file);
                        let mut writer = BufWriter::new(&out_file);
                        let mut it = reader.lines().peekable();
                        while let Some(line) = it.next() {
                            if it.peek().is_none() {
                                let s = match str::from_utf8(data) {
                                    Ok(v) => v,
                                    Err(e) => panic!("Invalid UTF-8 sequence {}", e),
                                };
                                let mut itt = s.lines().peekable();
                                while let Some(mline) = itt.next() {
                                    if !itt.peek().is_none()
                                        && !(mline.contains("kicad_symbol_lib")
                                            || mline.contains("version")
                                            || mline.contains("generator"))
                                    {
                                        writeln!(writer, "{}", mline).unwrap();
                                    }
                                }
                                writeln!(writer, ")").unwrap();
                            } else {
                                writeln!(writer, "{}", line.unwrap()).unwrap();
                            }
                        }
                    }
                    fs::rename(tmp_file.clone(), collected_symbol.clone()).unwrap();
                    let _ = fs::remove_file(tmp_file);
                } else if filename.contains(".kicad_mod") {
                    Self::write(
                        collected_footprints.join(format!(
                            "{}_{}",
                            save_dir.file_name().unwrap().to_str().unwrap(),
                            filename
                        )),
                        data.clone().to_vec(),
                    )?;
                }
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
