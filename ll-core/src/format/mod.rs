mod extractors;
use zip::read::ZipFile;
use super::error::LLResult;

pub use extractors::Files;

#[derive(Debug, Clone, PartialEq)]
pub enum ECAD {
    // * Keep these in alphabetical order
    D3, // 3D
    EAGLE,
    EASYEDA,
    KICAD,
    ZIP
}

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub name: String,
    pub ecad: ECAD,
    pub create_folder: bool,
    match_path: &'static str,
    ignore: Vec<&'static str>
}

impl Format {

    pub fn from<S: Into<String>>(format: S) -> Self {

        let f = format.into().to_lowercase();

        // * Keep these in alphabetical order
        match f.as_str() {
            "3d" => Self {
                name: f,
                ecad: ECAD::D3,
                create_folder: true,
                match_path: "3D",
                ignore: vec![]
            },
            "eagle" => Self {
                name: f,
                ecad: ECAD::EAGLE,
                create_folder: false,
                match_path: "EAGLE",
                ignore: vec!["Readme.html"]
            },
            "easyeda" => Self {
                name: f,
                ecad: ECAD::EASYEDA,
                create_folder: false,
                match_path: "EasyEDA",
                ignore: vec!["Readme.html"]
            },
            "kicad" => Self {
                name: f,
                ecad: ECAD::KICAD,
                create_folder: true,
                match_path: "KiCad",
                ignore: vec![]
            },
            "zip" => Self {
                name: f,
                ecad: ECAD::ZIP,
                create_folder: false,
                match_path: "",
                ignore: vec![]
            },
            _ => {
                eprintln!("{}#{}: Unknown format. Defaulting to ZIP!", std::file!(), std::line!());
                Self::from("zip")
            }
        }

    }

    pub fn extract(&self, files: &mut Files, file_path: String, item: &mut ZipFile) -> LLResult<()> {


        match &self.ecad {
            // * Keep these in alphabetical order
            ECAD::D3 => extractors::d3::extract(&self, files, file_path, item)?,
            ECAD::EAGLE => extractors::eagle::extract(&self, files, file_path, item)?,
            ECAD::EASYEDA => extractors::easyeda::extract(&self, files, file_path, item)?,
            ECAD::KICAD => extractors::kicad::extract(&self, files, file_path, item)?,
            ECAD::ZIP => unreachable!("ZIP not handled!")
            // ! NOTE: DO NOT ADD A _ => {} CATCHER HERE!
        };

        Ok(())
    }

}
