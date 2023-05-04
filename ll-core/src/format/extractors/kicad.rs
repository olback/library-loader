use super::*;

const FP_FOLDER : &str = "LibraryLoader.pretty";

pub fn extract(
    format: &Format,
    files: &mut Files,
    file_path: String,
    item: &mut ZipFile,
) -> Result<()> {
    //generic_extractor(format, files, file_path, item)
    let path = PathBuf::from(file_path);
    let base_name = path.file_name().unwrap().to_string_lossy().to_string();
    if let Some(ext) = &path.extension() {
        match ext.to_str() {
            //footprint and 3d files
            Some("kicad_mod") | Some("stl") | Some("stp") | Some("wrl") =>
                {
                    let mut f_data = Vec::<u8>::new();
                    item.read_to_end(&mut f_data)?;
                    files.insert(format!("{}/{}", FP_FOLDER, base_name), f_data);
                },
            _ => {
                println!("Unknown file type: {}", base_name);
            }
        }
    }

    Ok(())

}
