use std::collections::HashMap;

pub(super) use zip::read::ZipFile;
pub(super) use super::super::error::LLResult;
pub(super) use std::{io::Read, path::PathBuf};

pub type Files = HashMap::<String, Vec<u8>>;

pub trait ExtractorTrait {
    fn extract<S: Into<String>>(files: &mut Files, filepath: S, file: &mut ZipFile) -> LLResult<()>;
}


