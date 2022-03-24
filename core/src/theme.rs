use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

use crate::error::{Error, Result};

impl From<&PathBuf> for Module {
    fn from(path_buf: &PathBuf) -> Self {
        let content = fs::read(&path_buf).unwrap();
        let mut module = Module::from(content);
        module.view = path_buf.parent().unwrap().join(module.view);
        module
    }
}

impl From<&str> for Module {
    fn from(slice: &str) -> Self {
        Module::from(slice.as_bytes())
    }
}

impl From<&[u8]> for Module {
    fn from(slice: &[u8]) -> Self {
        serde_yaml::from_slice(slice).unwrap()
    }
}

impl From<Vec<u8>> for Module {
    fn from(slice: Vec<u8>) -> Self {
        serde_yaml::from_slice(&slice).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Module {
    pub view: PathBuf,
    #[serde(default)]
    pub fields: HashMap<String, Field>,
    #[serde(default)]
    pub areas: HashMap<String, Area>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    String,
    Integer,
    Boolean,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "accept", content = "components", rename_all = "snake_case")]
pub enum Area {
    All,
    Without(Vec<String>),
    With(Vec<String>),
}

#[derive(Debug)]
pub struct Theme {
    pub pages: HashMap<String, Module>,
    pub components: HashMap<String, Module>,
}

impl Theme {
    pub fn new(base_path: &str) -> Self {
        let pages = parse_modules(&format!("{}{}", base_path, "/pages/*/index.yaml"));
        let components = parse_modules(&format!("{}{}", base_path, "/components/*/index.yaml"));

        Self { pages, components }
    }
}

fn parse_modules(pattern: &str) -> HashMap<String, Module> {
    let mut result = HashMap::new();
    for entry in glob::glob(pattern).expect("Failed to read glob pattern") {
        let path = entry.expect("Failed to read path from glob");
        let module = Module::from(&path);
        let name = get_parent_name(&path).unwrap();
        result.insert(name, module);
    }

    result
}

fn get_parent_name(path: &PathBuf) -> Result<String> {
    let parent = path
        .parent()
        .ok_or(Error::theme("The module has no parent folder."))?;

    let name = parent
        .file_name()
        .ok_or(Error::theme("The parent folder has no name."))?;

    Ok(name.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_module() {
        let module_data = r#"
--- 
view: view.html
fields: 
  field_1: string
areas: 
  areas_1: 
    accept: without
    components: 
      - module_1
  areas_2: 
    accept: with
    components: 
      - module_1
  areas_3: 
    accept: all
        "#;

        let module = Module::from(module_data);

        assert_eq!(module.view, PathBuf::from("view.html"));
        assert_eq!(module.fields.len(), 1);
        assert_eq!(module.areas.len(), 3);
    }
}
