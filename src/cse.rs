use super::profile::Profile;
use super::epw::Epw;
use super::error::{LLResult, LLError};
use reqwest::{self, header};
use std::fs;

pub const COMPONENT_SEARCH_ENGINE_URL: &str = "http://componentsearchengine.com/ga/model.php?partID=";

pub struct CSE {
    auth: String
}

impl CSE {

    pub fn new(profile: Profile) -> Self {
        CSE {
            auth: profile.to_base64()
        }
    }

    pub fn get(&self, epw: Epw) -> LLResult<()> {

        let id = epw.id;
        let url = format!("{base}{id}", base = COMPONENT_SEARCH_ENGINE_URL, id = id);

        let client = reqwest::Client::new();
        let req = client.get(&url).header(header::AUTHORIZATION, format!("Basic {auth}", auth = &self.auth));
        let mut res = req.send()?;

        if !res.status().is_success() {

            return Err(LLError::new(format!("Error downloading file: {}", res.status())))

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
            println!("URL: {}", url);
            println!("Status: {}", res.status());
            println!("Headers {:#?}", res.headers());
            println!("Body length: {}", body.len());
            println!("Filename: {}", filename);
        }

        // TODO: Send filename and data to a new struct
        fs::write(format!("download/{}", &filename), &body)?;
        println!("Saving to download/{}", &filename);

        Ok(())

    }

}
