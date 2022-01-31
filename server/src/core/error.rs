use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum ErrorKind {
    Renderer,
    Module,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            Self::Renderer => "Render Error",
            Self::Module => "Module Error",
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            repr: Repr::Simple(kind),
        }
    }
}

enum Repr {
    Simple(ErrorKind),
    Stacked(ErrorKind, Box<dyn std::error::Error>),
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(kind) => f.debug_tuple("Simple").field(kind).finish(),
            Self::Stacked(kind, boxed_error) => f
                .debug_tuple("Stacked")
                .field(kind)
                .field(boxed_error)
                .finish(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            repr: Repr::Simple(kind),
        }
    }

    pub fn with_error<E>(kind: ErrorKind, stack: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            repr: Repr::Stacked(kind, stack.into()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jilo Core Error - ")?;
        match &self.repr {
            Repr::Simple(kind) => write!(f, "{}", kind.as_str()),
            Repr::Stacked(kind, error) => write!(f, "{} - {}", kind.as_str(), error),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
