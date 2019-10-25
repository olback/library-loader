use std::collections::HashMap;

pub(super) use zip::read::ZipFile;
pub(super) use super::super::error::LLResult;
pub(super) use std::{io::Read, path::PathBuf};

pub type Files = HashMap::<String, Vec<u8>>;

pub trait ExtractorTrait {
    fn extract<S: Into<String>>(files: &mut Files, file_path: S, item: &mut ZipFile) -> LLResult<()>;
}

pub(super) fn generic_extractor<S: Into<String>>(match_path: &str, files: &mut Files, file_path: S, item: &mut ZipFile) -> LLResult<()> {

    let fp = file_path.into();

    if fp.to_lowercase().contains(match_path) {
        let path = PathBuf::from(fp);
        let base_name = path.file_name().unwrap().to_string_lossy().to_string();
        let mut f_data = Vec::<u8>::new();
        item.read_to_end(&mut f_data)?;
        files.insert(base_name, f_data);
    }

    Ok(())

}
