// * Keep these in alphabetical order
pub mod d3;
pub mod eagle;
pub mod easyeda;
pub mod kicad;

pub(super) use {
    std::{collections::HashMap, io::Read, path::PathBuf},
    zip::read::ZipFile,
    {super::Format, crate::error::Result},
};

pub type Files = HashMap<String, Vec<u8>>;

pub(super) fn generic_extractor(
    format: &Format,
    files: &mut Files,
    file_path: String,
    item: &mut ZipFile,
) -> Result<()> {
    let file_path_lower = file_path.to_lowercase();

    // Ignore files
    for ignore in &format.ignore {
        if file_path_lower.contains(ignore.to_lowercase().as_str()) {
            return Ok(());
        }
    }

    if file_path_lower.contains(&format.match_path.to_lowercase()) {
        let path = PathBuf::from(file_path);
        let base_name = path.file_name().unwrap().to_string_lossy().to_string();
        let mut f_data = Vec::<u8>::new();
        item.read_to_end(&mut f_data)?;
        files.insert(base_name, f_data);
    }

    Ok(())
}
