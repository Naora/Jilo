use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use toml::value::Table;

use crate::core::{utils, website};

use super::Persist;

#[derive(Deserialize, Serialize, Debug)]
struct DataFile {
    template: String,
    areas: Vec<AreaData>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AreaData {
    name: String,
    modules: Vec<ModuleData>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ModuleData {
    name: String,
    data: Table,
}

#[derive(Debug)]
pub struct Locale {
    path: PathBuf,
    files: HashMap<String, DataFile>,
}

impl Locale {
    pub fn new(base_path: &str) -> Self {
        let mut files = HashMap::new();
        let pattern = format!("{}{}", base_path, "/**/*.toml");
        for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                let data_file: DataFile = utils::read_toml_file(&path)
                    .expect("Could not read toml file during loading locale files");

                if let Some(name) = &path.file_name() {
                    let name = name
                        .to_string_lossy()
                        .into_owned()
                        .strip_suffix(".toml")
                        .unwrap()
                        .to_owned();
                    files.insert(name, data_file);
                }
            }
        }

        Self {
            path: base_path.into(),
            files,
        }
    }
}

impl Persist for Locale {
    fn load(&self) -> Self {
        todo!()
    }

    fn save(&self) -> crate::core::error::Result<()> {
        todo!()
    }

    fn get_all_pages(&self) -> website::Pages {
        let mut result = HashMap::new();
        for (name, file) in &self.files {
            let modules = file
                .areas
                .iter()
                .map(|area| &area.modules)
                .flatten()
                .map(|m| m.name.to_owned())
                .collect();

            let page = website::Page {
                template: &file.template,
                modules,
                data: HashMap::new(),
            };
            result.insert(name.to_owned(), page);
        }
        result
    }
}
