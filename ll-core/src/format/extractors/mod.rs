// * Keep these in alphabetical order
pub mod kicad;

use std::io::Cursor;
pub(super) use {
    std::{collections::HashMap, io::Read, path::PathBuf},
    {super::Format, crate::error::Result},
};

pub type Files = HashMap<String, Vec<u8>>;

pub(super) fn generic_extractor(
    format: &Format,
    archive: &mut zip::ZipArchive<Cursor<&Vec<u8>>>,
) -> Result<HashMap<String, Vec<u8>>> {
    let mut files = Files::new();
    'file_loop: for i in 0..archive.len() {
        let mut item = archive.by_index(i)?;
        let file_path = item.name().to_string();
        let file_path_lower = file_path.to_lowercase();

        // Ignore files
        for ignore in &format.ignore {
            if file_path_lower.contains(ignore.to_lowercase().as_str()) {
                continue 'file_loop;
            }
        }

        if file_path_lower.contains(&format.match_path.to_lowercase()) {
            let path = PathBuf::from(file_path);
            let base_name = path.file_name().unwrap().to_string_lossy().to_string();
            let mut f_data = Vec::<u8>::new();
            item.read_to_end(&mut f_data)?;
            files.insert(base_name, f_data);
        }
    }

    Ok(files)
}
