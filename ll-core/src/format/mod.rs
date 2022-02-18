use {
    crate::error::{Error, Result},
    serde::{Deserialize, Serialize},
    std::{fmt, path::PathBuf},
    zip::read::ZipFile,
};

mod extractors;
pub use extractors::Files;

macro_rules! ecad {
    ([$(($variant:tt, $variant_literal:literal)),*]) => {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum ECAD {
            $(
                #[serde(rename = $variant_literal)]
                $variant,
            )*
        }

        impl ::core::convert::TryFrom<&str> for ECAD {
            type Error = Error;

            fn try_from(value: &str) -> ::core::result::Result<Self, Self::Error> {
                match value.to_lowercase().as_str() {
                    $(
                        $variant_literal => Ok(ECAD::$variant),
                    )*
                    _ => Err(Error::EcadNotFound),
                }
            }
        }

        impl ::core::fmt::Display for ECAD {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(
                    f,
                    "{}",
                    match &self {
                        $(
                            Self::$variant => $variant_literal,
                        )*
                    }
                )
            }
        }

    };
}

ecad!([
    (D3, "3d"),
    (DesignSpark, "designspark"),
    (Eagle, "eagle"),
    (EasyEDA, "easyeda"),
    (KiCad, "kicad"),
    (Zip, "zip")
]);

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
        // * Keep these in alphabetical order
        match ecad {
            ECAD::D3 => Self {
                output_path: output_path.into(),
                ecad,
                create_folder: true,
                match_path: "3D",
                ignore: vec![],
            },
            ECAD::DesignSpark => Self {
                output_path: output_path.into(),
                ecad,
                create_folder: false,
                match_path: "DesignSpark PCB",
                ignore: vec![],
            },
            ECAD::Eagle => Self {
                output_path: output_path.into(),
                ecad,
                create_folder: false,
                match_path: "EAGLE",
                ignore: vec!["Readme.html"],
            },
            ECAD::EasyEDA => Self {
                output_path: output_path.into(),
                ecad,
                create_folder: false,
                match_path: "EasyEDA",
                ignore: vec!["Readme.html"],
            },
            ECAD::KiCad => Self {
                output_path: output_path.into(),
                ecad,
                create_folder: true,
                match_path: "KiCad",
                ignore: vec![],
            },
            ECAD::Zip => Self {
                output_path: output_path.into(),
                ecad,
                create_folder: false,
                match_path: "",
                ignore: vec![],
            },
        }
    }

    pub fn extract(&self, files: &mut Files, file_path: String, item: &mut ZipFile) -> Result<()> {
        match &self.ecad {
            // * Keep these in alphabetical order
            ECAD::D3 => extractors::d3::extract(self, files, file_path, item)?,
            ECAD::DesignSpark => extractors::designspark::extract(self, files, file_path, item)?,
            ECAD::Eagle => extractors::eagle::extract(self, files, file_path, item)?,
            ECAD::EasyEDA => extractors::easyeda::extract(self, files, file_path, item)?,
            ECAD::KiCad => extractors::kicad::extract(self, files, file_path, item)?,
            ECAD::Zip => unreachable!("ZIP not handled!"),
            // ! NOTE: DO NOT ADD A _ => {} CATCHER HERE!
        };

        Ok(())
    }
}
