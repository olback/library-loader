use std::fmt;

#[derive(Debug)]
pub enum ClientKind {
    CLI,
    GUI,
}

impl fmt::Display for ClientKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::CLI => write!(f, "ll-cli"),
            Self::GUI => write!(f, "ll-gui"),
        }
    }
}
