use super::extractor_prelude::*;

pub struct Extractor;

impl ExtractorTrait for Extractor {

    fn extract<S: Into<String>>(files: &mut Files, file_path: S, item: &mut ZipFile) -> LLResult<()> {

        let fp = file_path.into();

        if fp.to_lowercase().contains("eagle") {
            let path = PathBuf::from(fp);
            let base_name = path.file_name().unwrap().to_string_lossy().to_string();
            let mut f_data = Vec::<u8>::new();
            item.read_to_end(&mut f_data)?;
            files.insert(base_name, f_data);
        }

        Ok(())
    }

}

