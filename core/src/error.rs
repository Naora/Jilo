use std::{error::Error as StdError, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptySummary,
    InvalidValue,
    Serde(serde_yaml::Error),
    Io(std::io::Error),
    Tera(tera::Error),
    ParseTheme,
    PageNotFound,
    TemplateNotFound,
    DuplicatedName,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::EmptySummary => write!(f, "no page found"),
            Error::Io(ref error) => write!(f, "io error: {}", error),
            Error::InvalidValue => write!(f, "value could not be determined"),
            Error::Serde(ref error) => write!(f, "serde error: {}", error),
            Error::Tera(..) => write!(f, "a tera error occured"),
            Error::ParseTheme => write!(f, "theme is not able to be parsed"),
            Error::PageNotFound => write!(f, "page is not in store"),
            Error::TemplateNotFound => write!(f, "template not found in theme"),
            Error::DuplicatedName => write!(f, "name already exist"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::Serde(ref error) => Some(error),
            Error::Tera(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(serde_error: serde_yaml::Error) -> Self {
        Error::Serde(serde_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::Io(io_error)
    }
}

impl From<tera::Error> for Error {
    fn from(tera_error: tera::Error) -> Self {
        Error::Tera(tera_error)
    }
}

impl From<glob::GlobError> for Error {
    fn from(glob_error: glob::GlobError) -> Self {
        Error::Io(glob_error.into_error())
    }
}
