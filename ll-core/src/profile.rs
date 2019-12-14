use base64;
use serde::{Serialize, Deserialize};
use reqwest::{self, header::AUTHORIZATION};
use crate::{new_err, consts, error::LLResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub password: String
}

impl Profile {

    // Constructors
    pub fn new<S: Into<String>>(u: S, p: S) -> Self {
        Profile {
            username: u.into(),
            password: p.into()
        }
    }

    pub fn prompt() -> Self {

        #[cfg(feature = "cli-auth")]
        {
            use std::io::Write;

            println!("Please log in to Component Search Engine:");

            print!("Username: ");
            std::io::stdout().flush().unwrap();
            let mut username = String::new();
            std::io::stdin().read_line(&mut username).expect("Failed to get user input");
            username = username.trim().to_owned();

            let password = rpassword::prompt_password_stdout("Password: ").expect("Failed to get password");

            Self::new(username, password)
        }

        #[cfg(not(feature = "cli-auth"))]
        Self::new("", "")

    }

    // Methods
    pub fn to_string(&self) -> String {

        format!("{u}:{p}", u = &self.username, p = &self.password)

    }

    pub fn to_base64(&self) -> String {

        let content = &self.to_string();
        base64::encode(&content)

    }

    pub fn try_auth(&self) -> LLResult<bool> {

        let client = reqwest::Client::new();
        let req = client.get(consts::TRY_AUTH_URL).header(AUTHORIZATION, format!("Basic {}", self.to_base64()));
        let res = req.send()?;

        if res.status().is_server_error() {
            return Err(new_err!("Server error"));
        }

        Ok(res.status().is_success())


    }

}
