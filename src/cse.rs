use super::profile::Profile;
use super::epw::Epw;
use super::error::{LLResult, LLError};
use super::cse_result::CSEResult;
use reqwest::{self, header};
use super::consts::COMPONENT_SEARCH_ENGINE_URL;

pub struct CSE {
    auth: String
}

impl CSE {

    pub fn new(profile: &Profile) -> Self {
        CSE {
            auth: profile.to_base64()
        }
    }

    pub fn get(&self, epw: Epw) -> LLResult<CSEResult> {

        let id = epw.id;
        let url = format!("{base}{id}", base = COMPONENT_SEARCH_ENGINE_URL, id = id);

        let client = reqwest::Client::new();
        let req = client.get(&url).header(header::AUTHORIZATION, format!("Basic {auth}", auth = &self.auth));
        let mut res = req.send()?;

        let res_header = match res.headers().get("content-type") {
            Some(v) => v.to_str().unwrap_or("unknown"),
            None => "unknown"
        };

        if !res.status().is_success() {

            return Err(LLError::new(format!("Error downloading file: {}", res.status())))

        } else if res_header != "application/x-zip" {

            return Err(LLError::new("Error downloading file: Could not determine content type"))

        }

        let mut body = Vec::<u8>::new();
        if res.copy_to(&mut body).is_err() {

            return Err(LLError::new("Error copying data from response"))

        }

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
            },
            None => String::from("unknown.zip")
        };

        #[cfg(debug_assertions)]
        {
            println!("-- Debug info from {file}#{line} --", file = std::file!(), line = std::line!());
            println!("URL: {}", url);
            println!("Status: {}", res.status());
            println!("Headers {:#?}", res.headers());
            println!("Body length: {}", body.len());
            println!("Filename: {}", filename);
            println!("-- End debug info from {file}#{line} --", file = std::file!(), line = std::line!());
        }

        Ok(CSEResult {
            filename: filename,
            data: body
        })

    }

}
