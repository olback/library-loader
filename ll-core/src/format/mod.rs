mod extractors;
mod processors;
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

#[derive(PartialEq,Debug, Clone)]
pub enum Output
{
    File(&'static str),
    Folder(&'static str)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub name: String,
    pub ecad: ECAD,
    pub create_folder: bool,
    match_path: Vec<&'static str>,
    output : Vec<Output>,
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
                match_path: vec!["3D"],
                output: vec![],
                ignore: vec![]
            },
            "eagle" => Self {
                name: f,
                ecad: ECAD::EAGLE,
                create_folder: false,
                match_path: vec!["EAGLE/", "/3D/"],
                output: vec![],
                ignore: vec!["Readme.html"]
            },
            "easyeda" => Self {
                name: f,
                ecad: ECAD::EASYEDA,
                create_folder: false,
                match_path: vec!["EasyEDA/", "/3D/"],
                output: vec![],
                ignore: vec!["Readme.html"]
            },
            "kicad" => Self {
                name: f,
                ecad: ECAD::KICAD,
                create_folder: false,
                match_path: vec!["KiCad/", "/3D/"],
                output: vec![Output::File("LibraryLoader.lib"), Output::File("LibraryLoader.dcm"), Output::Folder("LibraryLoader.pretty")],
                ignore: vec![]
            },
            "zip" => Self {
                name: f,
                ecad: ECAD::ZIP,
                create_folder: false,
                match_path: vec![""],
                output: vec![],
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

    pub fn process(&self, output_path : String, output_files : &mut Files, file_path : String, item : &mut Vec<u8>) -> LLResult<()>
    {
        match &self.ecad {
            // * Keep these in alphabetical order
            ECAD::D3 => processors::d3::process(&self, output_path, output_files, file_path, item)?,
            ECAD::EAGLE => processors::eagle::process(&self, output_path, output_files, file_path, item)?,
            ECAD::EASYEDA => processors::easyeda::process(&self, output_path, output_files, file_path, item)?,
            ECAD::KICAD => processors::kicad::process(&self, output_path, output_files, file_path, item)?,
            ECAD::ZIP => unreachable!("ZIP not handled!")
            // ! NOTE: DO NOT ADD A _ => {} CATCHER HERE!
        };

        Ok(())
    }
}
