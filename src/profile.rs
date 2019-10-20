use base64;

#[derive(Debug)]
pub struct Profile {
    pub username: String,
    pub password: String
}

impl Profile {

    pub fn new<S: Into<String>>(u: S, p: S) -> Self {
        Profile {
            username: u.into(),
            password: p.into()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{u}:{p}", u = &self.username, p = &self.password)
    }

    pub fn to_base64(&self) -> String {

        let content = &self.to_string();
        base64::encode(&content)

    }

}
