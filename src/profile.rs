use base64;
use serde::Deserialize;
use std::io::Write;
use rpassword;

#[derive(Debug, Clone, Deserialize)]
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

        println!("No login information found. Please log in to Component Search Engine:");

        print!("Username: ");
        std::io::stdout().flush().unwrap();
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).expect("Failed to get user input");
        username = username.trim().to_owned();

        let password = rpassword::prompt_password_stdout("Password: ").expect("Failed to get password");

        Self::new(username, password)

    }

    // Methods
    pub fn to_string(&self) -> String {

        format!("{u}:{p}", u = &self.username, p = &self.password)

    }

    pub fn to_base64(&self) -> String {

        let content = &self.to_string();
        base64::encode(&content)

    }

}
