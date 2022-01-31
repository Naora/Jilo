use std::path::{Path, PathBuf};

use serde::Deserialize;
use toml::value::Table;

use crate::utils;

use super::error::{Error, ErrorKind, Result};

fn get_default_view_path() -> String {
    "view.html".to_string()
}

fn get_default_style_path() -> String {
    "style.scss".to_string()
}

fn get_default_javascript_path() -> String {
    "index.js".to_string()
}

#[derive(Debug, Default, Deserialize)]
struct ModuleConfiguration {
    #[serde(default = "get_default_view_path")]
    view: String,
    #[serde(default = "get_default_style_path")]
    style: String,
    #[serde(default = "get_default_javascript_path")]
    entry: String,
    data: Table, // Todo use data to fetch props from modules
}

#[derive(Debug)]
pub struct Module {
    pub index: PathBuf,
    pub view: Option<PathBuf>,
    pub style: Option<PathBuf>,
    pub entry: Option<PathBuf>,
}

impl Module {
    pub fn new(base_path: &PathBuf) -> Self {
        let module: ModuleConfiguration =
            utils::read_toml_file(&base_path).expect("Toml could not be read");
        let parent = base_path.parent().unwrap_or(Path::new("/"));

        let view = parent
            .join(&module.view)
            .is_file()
            .then(|| parent.join(&module.view));

        let style = parent
            .join(&module.style)
            .is_file()
            .then(|| parent.join(&module.view));

        let entry = parent
            .join(&module.entry)
            .is_file()
            .then(|| parent.join(&module.view));

        Self {
            index: base_path.to_owned(),
            view,
            style,
            entry,
        }
    }

    pub fn get_module_name(&self) -> Result<String> {
        let parent = match self.index.parent() {
            Some(parent) => parent,
            None => return Err(Error::new(ErrorKind::Module)),
        };

        if parent.is_dir() {
            if let Some(directory) = parent.file_name() {
                let lossy = directory.to_string_lossy();
                return Ok(lossy.into_owned());
            }
        }
        Err(Error::new(ErrorKind::Module))
    }
}
