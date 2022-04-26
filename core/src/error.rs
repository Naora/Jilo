use std::{error::Error as StdError, fmt};

pub type Result<T> = std::result::Result<T, ErrorA>;

#[derive(Debug)]
pub enum ErrorA {
    EmptySummary,
    InvalidValue,
    Serde(serde_yaml::Error),
    Io(std::io::Error),
    Tera(tera::Error),
    ParseTheme,
}

impl fmt::Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorA::EmptySummary => write!(f, "no page found"),
            ErrorA::Io(..) => write!(f, "the file could not be open"),
            ErrorA::InvalidValue => write!(f, "value could not be determined"),
            ErrorA::Serde(_) => write!(f, "a serde error occured"),
            ErrorA::Tera(_) => write!(f, "a tera error occured"),
            ErrorA::ParseTheme => write!(f, "theme is not able to be parsed"),
        }
    }
}

impl StdError for ErrorA {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            ErrorA::Io(ref error) => Some(error),
            ErrorA::Serde(ref error) => Some(error),
            ErrorA::Tera(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_yaml::Error> for ErrorA {
    fn from(serde_error: serde_yaml::Error) -> Self {
        ErrorA::Serde(serde_error)
    }
}

impl From<std::io::Error> for ErrorA {
    fn from(io_error: std::io::Error) -> Self {
        ErrorA::Io(io_error)
    }
}

impl From<tera::Error> for ErrorA {
    fn from(tera_error: tera::Error) -> Self {
        ErrorA::Tera(tera_error)
    }
}

impl From<glob::GlobError> for ErrorA {
    fn from(glob_error: glob::GlobError) -> Self {
        ErrorA::Io(glob_error.into_error())
    }
}
