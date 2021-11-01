use {crate::error::Result, std::path::PathBuf, zip::read::ZipFile};

mod extractors;
pub use extractors::Files;

#[derive(Debug, Clone, PartialEq)]
pub enum ECAD {
    // * Keep these in alphabetical order
    D3, // 3D
    EAGLE,
    EASYEDA,
    KICAD,
    ZIP,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub output_path: PathBuf,
    pub name: String,
    pub ecad: ECAD,
    pub create_folder: bool,
    match_path: &'static str,
    ignore: Vec<&'static str>,
}

impl Format {
    pub fn from<S: Into<String>, P: Into<PathBuf>>(format: S, output_path: P) -> Self {
        let f = format.into().to_lowercase();

        // * Keep these in alphabetical order
        match f.as_str() {
            "3d" => Self {
                output_path: output_path.into(),
                name: f,
                ecad: ECAD::D3,
                create_folder: true,
                match_path: "3D",
                ignore: vec![],
            },
            "eagle" => Self {
                output_path: output_path.into(),
                name: f,
                ecad: ECAD::EAGLE,
                create_folder: false,
                match_path: "EAGLE",
                ignore: vec!["Readme.html"],
            },
            "easyeda" => Self {
                output_path: output_path.into(),
                name: f,
                ecad: ECAD::EASYEDA,
                create_folder: false,
                match_path: "EasyEDA",
                ignore: vec!["Readme.html"],
            },
            "kicad" => Self {
                output_path: output_path.into(),
                name: f,
                ecad: ECAD::KICAD,
                create_folder: true,
                match_path: "KiCad",
                ignore: vec![],
            },
            "zip" => Self {
                output_path: output_path.into(),
                name: f,
                ecad: ECAD::ZIP,
                create_folder: false,
                match_path: "",
                ignore: vec![],
            },
            _ => {
                eprintln!(
                    "{}#{}: Unknown format. Defaulting to ZIP!",
                    std::file!(),
                    std::line!()
                );
                Self::from("zip", output_path)
            }
        }
    }

    pub fn extract(&self, files: &mut Files, file_path: String, item: &mut ZipFile) -> Result<()> {
        match &self.ecad {
            // * Keep these in alphabetical order
            ECAD::D3 => extractors::d3::extract(&self, files, file_path, item)?,
            ECAD::EAGLE => extractors::eagle::extract(&self, files, file_path, item)?,
            ECAD::EASYEDA => extractors::easyeda::extract(&self, files, file_path, item)?,
            ECAD::KICAD => extractors::kicad::extract(&self, files, file_path, item)?,
            ECAD::ZIP => unreachable!("ZIP not handled!"), // ! NOTE: DO NOT ADD A _ => {} CATCHER HERE!
        };

        Ok(())
    }
}
