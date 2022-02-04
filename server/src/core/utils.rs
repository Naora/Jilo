use std::{fs, io, path::Path};

use serde::Deserialize;
use toml::de;

#[derive(Debug)]
pub enum ReadTomlFileError {
    IoError(io::Error),
    TomlError(de::Error),
}

pub fn read_toml_file<P, T>(path: P) -> Result<T, ReadTomlFileError>
where
    T: for<'a> Deserialize<'a>,
    P: AsRef<Path>,
{
    let content = match fs::read(path) {
        Ok(file) => file,
        Err(io_error) => return Err(ReadTomlFileError::IoError(io_error)),
    };
    let result = match toml::from_slice(&content) {
        Ok(de) => de,
        Err(toml_error) => return Err(ReadTomlFileError::TomlError(toml_error)),
    };
    Ok(result)
}
