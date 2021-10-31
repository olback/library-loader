// * Keep these in alphabetical order
pub mod eagle;
pub mod easyeda;
pub mod kicad;
pub mod d3;

use std::collections::HashMap;

pub(super) use zip::read::ZipFile;
pub(super) use std::{
    io::Read,
    path::PathBuf
};
pub(super) use super::{
    Format,
    super::LLResult
};

pub type Files = HashMap::<String, Vec<u8>>;

pub(super) fn generic_extractor(format: &Format, files: &mut Files, file_path: String, item: &mut ZipFile) -> LLResult<()> {

    let file_path_lower = file_path.to_lowercase();

    // Ignore files
    for ignore in &format.ignore {

        if file_path_lower.contains(ignore.to_lowercase().as_str()) {

            return Ok(())

        }

    }

    let path = PathBuf::from(file_path);
    for paths_to_extract in &format.match_path {
        if file_path_lower.contains(paths_to_extract.to_lowercase().as_str()) {
            let base_name = path.file_name().unwrap().to_string_lossy().to_string();
            let mut f_data = Vec::<u8>::new();
            item.read_to_end(&mut f_data)?;
            files.insert(base_name, f_data);
        }
    }

    Ok(())

}
