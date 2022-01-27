use std::path::{Path, PathBuf};

use serde::Deserialize;
use toml::value::Table;

use crate::utils;

fn default_view_path() -> String {
    "view.html".to_string()
}

fn default_style_path() -> String {
    "style.scss".to_string()
}

fn default_javascript_path() -> String {
    "index.js".to_string()
}

#[derive(Debug, Default, Deserialize)]
struct ModuleConfiguration {
    #[serde(default = "default_view_path")]
    view: String,
    #[serde(default = "default_style_path")]
    style: String,
    #[serde(default = "default_javascript_path")]
    entry: String,
    data: Table,
}

#[derive(Debug)]
pub struct Module {
    view: Option<PathBuf>,
    style: Option<PathBuf>,
    entry: Option<PathBuf>,
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

        Self { view, style, entry }
    }
}
