mod extractors;
use zip::read::ZipFile;
use super::error::LLResult;

pub use extractors::Files;

#[derive(Debug, Clone, PartialEq)]
pub enum ECAD {
    // * Keep these in alphabetical order
    EAGLE,
    EASYEDA,
    KICAD,
    ZIP
}

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub ecad: ECAD,
    pub create_folder: bool,
    match_path: Vec<&'static str>,
    ignore: Vec<&'static str>
}

impl Format {

    pub fn from<S: Into<String>>(format: S) -> Self {

        let f = format.into().to_lowercase();

        // * Keep these in alphabetical order
        match f.as_str() {
            "eagle" => Self {
                ecad: ECAD::EAGLE,
                create_folder: false,
                match_path: vec!["EAGLE/", "/3D/"],
                ignore: vec!["Readme.html"]
            },
            "easyeda" => Self {
                ecad: ECAD::EASYEDA,
                create_folder: false,
                match_path: vec!["EasyEDA/", "/3D/"],
                ignore: vec!["Readme.html"]
            },
            "kicad" => Self {
                ecad: ECAD::KICAD,
                create_folder: true,
                match_path: vec!["KiCad/", "/3D/"],
                ignore: vec![]
            },
            "zip" => Self {
                ecad: ECAD::ZIP,
                create_folder: false,
                match_path: vec![""],
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
            ECAD::EAGLE => extractors::eagle::extract(&self, files, file_path, item)?,
            ECAD::EASYEDA => extractors::easyeda::extract(&self, files, file_path, item)?,
            ECAD::KICAD => extractors::kicad::extract(&self, files, file_path, item)?,
            ECAD::ZIP => panic!("This should be unreachable!")
            // ! NOTE: DO NOT ADD A _ => {} CATCHER HERE!
        };

        Ok(())
    }

}
