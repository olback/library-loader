use {
    crate::{
        consts::COMPONENT_SEARCH_ENGINE_URL,
        epw::Epw,
        error::{self, Error},
        format::{Files, Format, ECAD},
    },
    reqwest::header,
    std::{collections::HashMap, path::PathBuf, sync::Arc},
};

mod result;
pub use result::Result;

pub struct CSE {
    auth: String,
    formats: Arc<Vec<Format>>,
}

impl CSE {
    pub fn new(token: String, formats: Arc<Vec<Format>>) -> Self {
        CSE {
            auth: token,
            formats,
        }
    }

    pub fn get(&self, epw: Epw) -> error::Result<Result> {
        let id = epw.id;
        let url = format!("{base}{id}", base = COMPONENT_SEARCH_ENGINE_URL, id = id);

        let client = reqwest::blocking::Client::new();
        let req = client
            .get(&url)
            .header(
                header::AUTHORIZATION,
                format!("Basic {auth}", auth = &self.auth),
            )
            .header(
                header::USER_AGENT,
                format!(
                    "Library Loader {} github.com/olback/library-loader",
                    env!("CARGO_PKG_VERSION")
                ),
            );
        let mut res = req.send()?;

        let res_header = match res.headers().get("content-type") {
            Some(v) => v.to_str().unwrap_or("unknown"),
            None => "unknown",
        };

        if !res.status().is_success() {
            return Err(Error::ServerError(
                res.status().as_str(),
                res.status().as_u16(),
            ));
        } else if res_header != "application/x-zip" {
            return Err(Error::Other("Error downloading file: Could not determine content type. This may be because the terms have changed. Log in at componentsearchengine.com and accept the new terms."));
        }

        let mut body = Vec::<u8>::new();
        res.copy_to(&mut body)?;

        let filename = match res.headers().get("content-disposition") {
            Some(v) => {
                let content_disposition = String::from_utf8_lossy(v.as_bytes()).to_string();
                content_disposition
                    .replace("attachment;", "")
                    .trim()
                    .replace("filename=", "")
                    .replace("\"", "")
                    .trim()
                    .to_string()
            }
            None => String::from("unknown.zip"),
        };

        #[cfg(debug_assertions)]
        {
            println!(
                "-- Debug info from {file}#{line} --",
                file = std::file!(),
                line = std::line!()
            );
            println!("URL: {}", url);
            println!("Status: {}", res.status());
            println!("Headers {:#?}", res.headers());
            println!("Body length: {}", body.len());
            println!("Filename: {}", filename);
            println!(
                "-- End debug info from {file}#{line} --",
                file = std::file!(),
                line = std::line!()
            );
        }

        // FIXME:TODO: Use formats vec here!

        if &self.config.settings.format.ecad == &ECAD::ZIP {
            let mut files: Files = HashMap::new();
            files.insert(filename, body);

            Ok(Result {
                output_path: self.config.settings.output_path.to_owned(),
                files: files,
            })
        } else {
            let lib_name = match filename.starts_with("LIB_") {
                true => filename.as_str()[4..].replace(".zip", ""),
                false => filename.replace(".zip", ""),
            };

            self.unzip(lib_name, body)
        }
    }

    fn unzip(&self, lib_name: String, data: Vec<u8>) -> error::Result<Result> {
        let reader = std::io::Cursor::new(&data);
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut files: Files = HashMap::new();

        for i in 0..archive.len() {
            let mut item = archive.by_index(i)?;
            let filename = String::from(item.name());

            &self
                .config
                .settings
                .format
                .extract(&mut files, filename, &mut item)?;
        }

        let path = match &self.config.settings.format.create_folder {
            true => PathBuf::from(&self.config.settings.output_path).join(lib_name),
            false => PathBuf::from(&self.config.settings.output_path),
        };

        Ok(Result {
            output_path: path.to_string_lossy().to_string(),
            files: files,
        })
    }
}
