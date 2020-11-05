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
    super::LLResult,
    super::LLError
};

pub type Files = HashMap::<String, Vec<u8>>;

pub(super) fn generic_processor(format: &Format, output_path : String, output_files : &mut Files, file_path : String, item : &mut Vec<u8>) -> LLResult<()> {

    output_files.insert(file_path, item.clone());

    Ok(())
}
