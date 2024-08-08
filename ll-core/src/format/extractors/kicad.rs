use super::*;

pub fn extract(
    format: &Format,
    files: &mut Files,
    file_path: String,
    item: &mut ZipFile,
) -> Result<()> {
    generic_extractor(format, files, file_path, item)
}
