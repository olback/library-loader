use reqwest;

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

impl From<std::io::Error> for LLError {
    fn from(err: std::io::Error) -> LLError {
        Self::new(format!("{}", err))
    }
}

impl From<std::num::ParseIntError> for LLError {
    fn from(err: std::num::ParseIntError) -> LLError {
        Self::new(format!("{}", err))
    }
}

// impl From<std::option::NoneError> for LLError {
//     fn from (err: std::option::NoneError) -> LLError {
//         Self::new(format!("{}", err))
//     }
// }

impl From<reqwest::Error> for LLError {
    fn from(err: reqwest::Error) -> LLError {
        Self::new(format!("{}", err))
    }
}

// impl From<http::header::value::ToStrError> for LLError {
//     fn from(err: reqwest::Error) -> LLError {
//         Self::new(format!("{}", err))
//     }
// }
