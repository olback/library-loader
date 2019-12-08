use super::*;

pub fn extract(format: &Format, files: &mut Files, file_path: String, item: &mut ZipFile) -> LLResult<()> {


    generic_extractor(format, files, file_path, item)

}

