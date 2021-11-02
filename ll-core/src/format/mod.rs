use {
    crate::error::{Error, Result},
    serde::{Deserialize, Serialize},
    std::{fmt, path::PathBuf},
    zip::read::ZipFile,
};

mod extractors;
pub use extractors::Files;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ECAD {
    // * Keep these in alphabetical order
    #[serde(rename = "3d")]
    D3, // 3D
    #[serde(rename = "eagle")]
    EAGLE,
    #[serde(rename = "easyeda")]
    EASYEDA,
    #[serde(rename = "kicad")]
    KICAD,
    #[serde(rename = "zip")]
    ZIP,
}

impl TryFrom<&str> for ECAD {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "3d" => Ok(ECAD::D3),
            "eagle" => Ok(ECAD::EAGLE),
            "easyeda" => Ok(ECAD::EASYEDA),
            "kicad" => Ok(ECAD::KICAD),
            "zip" => Ok(ECAD::ZIP),
            _ => Err(Error::EcadNotFound),
        }
    }
}

impl fmt::Display for ECAD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::D3 => "3d",
                Self::EAGLE => "eagle",
                Self::EASYEDA => "easyeda",
                Self::KICAD => "kicad",
                Self::ZIP => "zip",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub output_path: PathBuf,
    // pub name: String,
    pub ecad: ECAD,
    pub create_folder: bool,
    match_path: &'static str,
    ignore: Vec<&'static str>,
}

impl Format {
    pub fn from_ecad<P: Into<PathBuf>>(ecad: ECAD, output_path: P) -> Self {
        // let f = format.into().to_lowercase();

        // * Keep these in alphabetical order
        match ecad {
            ECAD::D3 => Self {
                output_path: output_path.into(),
                // name: f,
                ecad: ECAD::D3,
                create_folder: true,
                match_path: "3D",
                ignore: vec![],
            },
            ECAD::EAGLE => Self {
                output_path: output_path.into(),
                // name: f,
                ecad: ECAD::EAGLE,
                create_folder: false,
                match_path: "EAGLE",
                ignore: vec!["Readme.html"],
            },
            ECAD::EASYEDA => Self {
                output_path: output_path.into(),
                // name: f,
                ecad: ECAD::EASYEDA,
                create_folder: false,
                match_path: "EasyEDA",
                ignore: vec!["Readme.html"],
            },
            ECAD::KICAD => Self {
                output_path: output_path.into(),
                // name: f,
                ecad: ECAD::KICAD,
                create_folder: true,
                match_path: "KiCad",
                ignore: vec![],
            },
            ECAD::ZIP => Self {
                output_path: output_path.into(),
                // name: f,
                ecad: ECAD::ZIP,
                create_folder: false,
                match_path: "",
                ignore: vec![],
            },
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
