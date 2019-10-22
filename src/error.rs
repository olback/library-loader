pub type LLResult<T> = std::result::Result<T, LLError>;

#[derive(Debug)]
pub struct LLError {
    pub cause: String
}

impl LLError {

    pub fn new<S: Into<String>>(cause: S) -> Self {
        LLError {
            cause: cause.into()
        }
    }

}

impl std::fmt::Display for LLError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl From<std::io::Error> for LLError {
    fn from(err: std::io::Error) -> Self {
        Self::new(format!("{}", err))
    }
}

impl From<std::num::ParseIntError> for LLError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::new(format!("{}", err))
    }
}

impl From<reqwest::Error> for LLError {
    fn from(err: reqwest::Error) -> Self {
        Self::new(format!("{}", err))
    }
}

impl From<toml::de::Error> for LLError {
    fn from(err: toml::de::Error) -> Self {
        Self::new(format!("{}", err))
    }
}

impl From <notify::Error> for LLError {
    fn from(err: notify::Error) -> Self {
        Self::new(format!("{}", err))
    }
}
