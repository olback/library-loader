use super::*;

const LIBRARY : &str = "LibraryLoader.lib";
const DOCLIB : &str = "LibraryLoader.dcm";
const FP_FOLDER : &str = "LibraryLoader.pretty";
const U3D_FOLDER : &str = "LibraryLoader-3dmodels";

pub fn process(format: &Format, output_path : String, output_files : &mut Files, file_name : String, item : &mut Vec<u8>) -> LLResult<()> {

    let file_path = PathBuf::from(output_path.clone()).join(file_name.clone());
    if let Some(ext) = &file_path.extension()
    {
        match ext.to_str()
        {
            Some("kicad_mod") =>
            {
                output_files.insert(format!("{}/{}", FP_FOLDER, file_name), item.clone());
            },
            Some("lib") =>
            {
                let chars = match std::str::from_utf8(&item[..])
                    {
                        Ok(v) => v,
                        Err(e) => return Err(
                            LLError::new(format!("Could not convert file to valid utf8: {} ({})", e, file_name), "ll-core/src/format/processors/kicad.rs", 24)
                        )
                    };
                
                let mut d_out = Vec::new();

                let enddefs : Vec<_> = chars.match_indices("ENDDEF").collect();
                let mut start = 0;
                for (idx,_) in enddefs
                {
                    let matching_def = chars[start..idx].match_indices("DEF").collect::<Vec<_>>()[0].0;
                    d_out.extend_from_slice(&chars[matching_def..idx+6].as_bytes());
                    start = idx+7;
                }

                if let Some(f) = output_files.get_mut(LIBRARY) {
                    f.append(&mut d_out);
                }
                else {
                    // Create entry in output_files for library
                    let mut file_data = Vec::new();

                    // Load in from possibly existing file
                    let fn_lib = PathBuf::from(output_path).join(LIBRARY);

                    if fn_lib.exists()
                    {
                        let mut f_lib = std::fs::File::open(fn_lib)?;

                        f_lib.read_to_end(&mut file_data)?;
                    }
                    else {
                        file_data.extend_from_slice(b"EESchema-LIBRARY Version 2.3\n#encoding utf-8");
                    }
                    file_data.push(b'\n');
                    file_data.append(&mut d_out);
                    
                    output_files.insert(LIBRARY.to_string(), file_data);
                }
            },
            Some("dcm") =>
            {
                let chars = match std::str::from_utf8(&item[..])
                    {
                        Ok(v) => v,
                        Err(e) => return Err(
                            LLError::new(format!("Could not convert file to valid utf8: {} ({})", e, file_name), "ll-core/src/format/processors/kicad.rs", 68)
                        )
                    };
                
                let mut d_out = Vec::new();

                let endcmps : Vec<_> = chars.match_indices("$ENDCMP").collect();
                let mut start = 0;
                for (idx,_) in endcmps
                {
                    let matching_cmp = chars[start..idx].match_indices("$CMP").collect::<Vec<_>>()[0].0;
                    d_out.extend_from_slice(&chars[matching_cmp..idx+7].as_bytes());
                    d_out.push(b'\n');
                    start = idx+8;
                }

                if let Some(f) = output_files.get_mut(DOCLIB) {
                    f.append(&mut d_out);
                }
                else {
                    // Create entry in output_files for library
                    let mut file_data = Vec::new();

                    // Load in from possibly existing file
                    let fn_lib = PathBuf::from(output_path).join(DOCLIB);

                    if fn_lib.exists()
                    {
                        let mut f_lib = std::fs::File::open(fn_lib)?;

                        f_lib.read_to_end(&mut file_data)?;
                    }
                    else {
                        file_data.extend_from_slice(b"EESchema-DOCLIB  Version 2.0\n");
                    }

                    file_data.append(&mut d_out);
                    
                    output_files.insert(DOCLIB.to_string(), file_data);
                }
            },
            Some("stl") | Some("stp") | Some("wrl") => 
            {
                // 3D Files
                let mut folder = PathBuf::from(file_name.clone());
                folder.set_extension("3dshapes");
                output_files.insert(format!("{}/{}/{}",U3D_FOLDER,folder.to_string_lossy(), file_name), item.clone());
            },
            Some("mod") => 
            {
                //Obsolete footprint module file
            }
            _ => println!("Unknown file type: {}", file_name)
        }
    }

    Ok(())
    /*generic_processor(format, output_path, output_files, file_name, item)*/
}

