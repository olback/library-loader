pub mod eagle;
pub mod easyeda;
pub mod kicad;

pub(super) mod extractor_prelude;
pub use extractor_prelude::{
    ExtractorTrait as Extractor,
    Files
};

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    EAGLE,
    EASYEDA,
    KICAD,
    ZIP
}

impl Format {

    pub fn from<S: Into<String>>(format: S) -> Self {

        let f = format.into().to_lowercase();

        match f.as_str() {
            "eagle" => Self::EAGLE,
            "easyeda" => Self::EASYEDA,
            "kicad" => Self::KICAD,
            "zip" => Self::ZIP,
            _ => {
                eprintln!("Unknown format. Defaulting to ZIP!");
                Self::ZIP
            }
        }

    }

}
