use std::{fs, path::Path};

const OUT_PATH: &str = "out";

pub fn output_dir() {

    if Path::new(OUT_PATH).exists() {
        fs::remove_dir_all(OUT_PATH).unwrap();
        fs::create_dir_all(OUT_PATH).unwrap();
    } else {
        fs::create_dir_all(OUT_PATH).unwrap();
    }

}
