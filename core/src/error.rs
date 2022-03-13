use std::{
    error,
    fmt::{self, Display},
    result,
};

pub type Result<'a, T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    Theme,
    Renderer,
    Site,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Theme => write!(f, "Theme"),
            ErrorKind::Renderer => write!(f, "Renderer"),
            ErrorKind::Site => write!(f, "Site"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new<S>(kind: ErrorKind, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub(crate) fn renderer<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Error::new(ErrorKind::Renderer, message)
    }

    pub(crate) fn theme<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Error::new(ErrorKind::Theme, message)
    }

    pub(crate) fn site<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::new(ErrorKind::Site, message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n Jilo Error")?;
        write!(f, "\n Kind {}", self.kind)?;
        write!(f, "\n Error is : \n{}", self.message)?;

        Ok(())
    }
}

impl error::Error for Error {}
