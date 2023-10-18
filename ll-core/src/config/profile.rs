use {
    crate::{
        consts,
        error::{Error, Result},
    },
    reqwest::header::AUTHORIZATION,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub password: String,
}

impl Profile {
    pub fn token(&self) -> String {
        data_encoding::BASE64.encode(format!("{}:{}", self.username, self.password).as_bytes())
    }

    pub fn is_empty(&self) -> bool {
        self.username.is_empty() || self.password.is_empty()
    }

    pub fn try_auth(&self) -> Result<bool> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(consts::TRY_AUTH_URL)
            .header(AUTHORIZATION, format!("Basic {}", self.token()));
        let res = req.send()?;

        if res.status().is_server_error() {
            return Err(Error::ServerError(res.status().as_u16()));
        }

        Ok(res.status().is_success())
    }
}
