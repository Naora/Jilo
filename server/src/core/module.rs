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
    view: String,
    style: String,
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
        let page: ModuleConfiguration =
            utils::read_toml_file(&base_path).expect("Toml could not be read");
        let parent = base_path.parent().unwrap_or(Path::new("/"));
        let view = parent
            .join(&page.view)
            .is_file()
            .then(|| parent.join(&page.view));

        let style = parent
            .join(&page.style)
            .is_file()
            .then(|| parent.join(&page.view));

        let entry = parent
            .join(&page.entry)
            .is_file()
            .then(|| parent.join(&page.view));

        Self { view, style, entry }
    }
}
