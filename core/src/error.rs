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
    Store,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Theme => write!(f, "Theme"),
            ErrorKind::Renderer => write!(f, "Renderer"),
            ErrorKind::Store => write!(f, "Store"),
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

    pub(crate) fn store<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::new(ErrorKind::Store, message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Jilo Error")?;
        writeln!(f, "Kind {}", self.kind)?;
        writeln!(f, "Error is :")?;
        writeln!(f, "{}", self.message)?;

        Ok(())
    }
}

impl error::Error for Error {}
