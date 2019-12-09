use super::{
    is_debug,
    impl_from
};

pub type LLResult<T> = std::result::Result<T, LLError>;

#[derive(Debug)]
pub struct LLError {
    cause: String,
    file: String,
    line: u32
}

impl LLError {

    pub fn new<S: Into<String>>(cause: S, file: &str, line: u32) -> LLError {
        LLError {
            cause: cause.into(),
            file: String::from(file),
            line: line
        }
    }

}

impl std::fmt::Display for LLError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        if is_debug!() {
            return write!(f, "{}#{}: {}", self.file, self.line, self.cause);
        } else {
            return write!(f, "{}", self.cause);
        }

    }
}

impl_from!(std::io::Error);
impl_from!(std::num::ParseIntError);
impl_from!(reqwest::Error);
impl_from!(toml::de::Error);
impl_from!(notify::Error);
impl_from!(zip::result::ZipError);
