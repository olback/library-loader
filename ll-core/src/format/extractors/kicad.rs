use super::*;
use std::fs;
use std::fs::File;
use std::io::{BufRead, Cursor, Seek, SeekFrom, Write};

pub fn extract(
    format: &Format,
    archive: &mut zip::ZipArchive<Cursor<&Vec<u8>>>,
) -> Result<HashMap<String, Vec<u8>>> {
    let fp_folder_str = format!("{}.pretty", format.name);

    //ensure we have the footprint library folder
    let footprint_folder = PathBuf::from(&format.output_path).join(fp_folder_str.clone());
    if !footprint_folder.exists() {
        fs::create_dir_all(footprint_folder.clone())?;
    }

    //ensure the symbol library exists
    let fn_lib = PathBuf::from(&format.output_path).join(format!("{}.kicad_sym", format.name));

    if !fn_lib.exists() {
        fs::write(
            &fn_lib,
            "(kicad_symbol_lib (version 20211014) (generator library-loader)\n)\n",
        )
        .expect("Unable to create symbol library file");
    }

    let mut symbols: Vec<String> = Vec::new();

    for i in 0..archive.len() {
        let mut item = archive.by_index(i)?;
        let name = item.name();
        let path = PathBuf::from(name);
        let base_name = path.file_name().unwrap().to_string_lossy().to_string();
        if let Some(ext) = &path.extension() {
            match ext.to_str() {
                //footprint and 3d files are copied first
                Some("kicad_mod") | Some("stl") | Some("stp") | Some("wrl") => {
                    let mut f_data = Vec::<u8>::new();
                    item.read_to_end(&mut f_data)?;
                    let mut f = File::create(footprint_folder.join(base_name))?;
                    f.write_all(&f_data)?;
                }
                Some("kicad_sym") => {
                    //save these to add later, so KiCad will be able to load the footprints right away
                    symbols.push(name.to_owned());
                }
                _ => {
                    // ignore all other files
                }
            }
        }
    }

    let mut f = File::options().read(true).write(true).open(&fn_lib)?;
    f.seek(SeekFrom::End(-2))?;

    for symbol_file in symbols {
        let mut f_data = Vec::<u8>::new();
        let mut item = archive.by_name(&symbol_file)?;
        item.read_to_end(&mut f_data)?;
        let mut lines: Vec<String> = (&f_data[..])
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        let end = &lines.len() - 1;
        for i in 0..end {
            //this is necessary to point symbols to correct footprint library
            let parts = lines[i].split_whitespace().collect::<Vec<_>>();
            if parts.len() >= 2 && parts[0] == "(property" && parts[1] == "\"Footprint\"" {
                let footprint_name = &parts[2][1..(parts[2].len() - 1)];
                lines[i] = lines[i].replace(
                    footprint_name,
                    &*format!("{}:{}", format.name, &footprint_name),
                );
            }
        }
        for line in &lines[1..end] {
            f.write_all(line.as_bytes())?;
            f.write_all("\n".as_bytes())?;
        }
    }
    f.write_all(")\n".as_bytes())?;

    Ok(Files::new())
}
