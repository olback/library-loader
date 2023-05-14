use super::*;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use crate::Error;


pub fn extract(
    format: &Format,
    files: &mut Files,
    file_path: String,
    item: &mut ZipFile,
) -> Result<()> {
    //generic_extractor(format, files, file_path, item)
    let path = PathBuf::from(file_path);
    let base_name = path.file_name().unwrap().to_string_lossy().to_string();

    let fp_folder_str = format!("{}.pretty", format.name);

    //ensure we have the footprint library folder
    let footprint_folder = PathBuf::from(&format.output_path).join(fp_folder_str.clone());
    if !footprint_folder.exists() {
        fs::create_dir_all(footprint_folder)?;
    }

    //ensure the symbol library exists
    let fn_lib = PathBuf::from(&format.output_path).join(format!("{}.kicad_sym", format.name));

    if !fn_lib.exists() {
        fs::write(&fn_lib, "(kicad_symbol_lib (version 20211014) (generator library-loader)\n)\n").expect("Unable to create symbol library file");
    }

    if let Some(ext) = &path.extension() {
        match ext.to_str() {
            //footprint and 3d files
            Some("kicad_mod") | Some("stl") | Some("stp") | Some("wrl") =>
                {
                    let mut f_data = Vec::<u8>::new();
                    item.read_to_end(&mut f_data)?;
                    files.insert(format!("{}/{}", fp_folder_str.clone(), base_name), f_data);
                },
            Some("kicad_sym") =>
                {
                    let mut f_data = Vec::<u8>::new();
                    item.read_to_end(&mut f_data)?;
                    let mut lines: Vec<String> = (&f_data[..]).lines().map(|l| l.expect("Could not parse line")).collect();
                    let end = &lines.len()-1;
                    let mut pos = 0;
                    for (i, line) in (lines[1..end]).iter_mut().enumerate() {
                        let parts = line.split_whitespace().collect::<Vec<_>>();
                        if parts.len() >= 2 && parts[0] == "(property" && parts[1] == "\"Footprint\"" {
                            pos = i + 1;
                        }
                    }
                    let parts = lines[pos].split_whitespace().collect::<Vec<_>>();
                    let footprint_name = &parts[2][1..(parts[2].len()-1)];
                    lines[pos] = lines[pos].replace(footprint_name, &*format!("LibraryLoader:{}", &footprint_name));


                    let mut f = File::options().read(true).write(true).open(&fn_lib)?;
                    f.seek(SeekFrom::End(-2))?;

                    for line in &lines[1..end] {
                        f.write_all(line.as_bytes())?;
                        f.write_all("\n".as_bytes())?;
                    }
                    f.write_all(")\n".as_bytes())?;
                },
            _ => {
                // ignore
            }
        }
    }

    Ok(())

}
