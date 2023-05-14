use std::io::Cursor;
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
    pub name: String,
    pub ecad: ECAD,
    pub create_folder: bool,
    match_path: &'static str,
    ignore: Vec<&'static str>,
}

impl Format {
    pub fn from_ecad<P: Into<PathBuf>>(name: &String, ecad: ECAD, output_path: P) -> Self {
        //defaults
        let mut fmt = Self {
            output_path: output_path.into(),
            name: (*name).clone(),
            ecad,
            create_folder: false,
            match_path: "",
            ignore: vec![],
        };
        // specific overrides, keep these in alphabetical order
        match fmt.ecad {
            ECAD::D3 => {
                fmt.create_folder = true;
                fmt.match_path = "3D";
            },
            ECAD::DesignSpark => {
                fmt.match_path = "DesignSpark PCB";
            },
            ECAD::Eagle => {
                fmt.match_path = "EAGLE";
                fmt.ignore = vec!["Readme.html"];
            },
            ECAD::EasyEDA => {
                fmt.match_path = "EasyEDA";
                fmt.ignore = vec!["Readme.html"];
            },
            ECAD::KiCad => {
                fmt.match_path = "KiCad";
            },
            ECAD::Zip => {
                //no changes
            },
        }
        return fmt
    }

    pub fn extract(&self, archive: &mut zip::ZipArchive<Cursor<&Vec<u8>>>) -> Result<()> {
        match &self.ecad {
            // * Keep these in alphabetical order
            ECAD::D3 => extractors::d3::extract(&self, files, file_path, item)?,
            ECAD::DesignSpark => extractors::designspark::extract(&self, files, file_path, item)?,
            ECAD::Eagle => extractors::eagle::extract(&self, files, file_path, item)?,
            ECAD::EasyEDA => extractors::easyeda::extract(&self, files, file_path, item)?,
            ECAD::KiCad => extractors::kicad::extract(&self, archive)?,
            ECAD::Zip => unreachable!("ZIP not handled!"),
            // ! NOTE: DO NOT ADD A _ => {} CATCHER HERE!
        };

        Ok(())
    }
}
