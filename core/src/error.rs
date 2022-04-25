use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub struct Error<T>
where
    T: fmt::Debug + fmt::Display + Sync + Send,
{
    kind: T,
    source: Option<Box<dyn StdError + Sync + Send>>,
}

impl<T> Error<T>
where
    T: fmt::Debug + fmt::Display + Sync + Send,
{
    pub fn new(kind: T) -> Self {
        Self { kind, source: None }
    }

    pub fn with<S>(kind: T, source: S) -> Self
    where
        S: StdError + 'static + Sync + Send,
    {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }
}

impl<T> fmt::Display for Error<T>
where
    T: fmt::Debug + fmt::Display + Sync + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "jilo error - {}", self.kind)
    }
}

impl<T> StdError for Error<T>
where
    T: fmt::Debug + fmt::Display + Sync + Send,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.source {
            Some(ref error) => Some(error.as_ref()),
            None => None,
        }
    }
}

// Theme errors variants

display_enum!(ThemeErrorKind, {
    ParsePathError => "could not convert path into valid utf8",
    CanonicalError => "no valid canonical path",
    ThemeParseError => "could not find relative path to theme"
});

pub type ThemeError = Error<ThemeErrorKind>;

display_enum!(SiteErrorKind, {
    PageRenderError => "page could not be rendered"
});

pub type SiteError = Error<SiteErrorKind>;

display_enum!(RenderErrorKind, {
    LoadError => "unable to load theme into render engine",
    RenderModuleError => "render failed"
});

pub type RenderError = Error<RenderErrorKind>;

display_enum!(StoreErrorKind, {
    SummaryError => "could not find all pages",
    LoadPageError => "page could not be loaded",
    PersistError => "unable to save pages",
    IoError => "could not open file"
});

pub type StoreError = Error<StoreErrorKind>;
