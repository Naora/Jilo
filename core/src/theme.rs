use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

use crate::error::{Error, Result};

impl From<&PathBuf> for Template {
    fn from(path_buf: &PathBuf) -> Self {
        let content = fs::read(&path_buf).unwrap();
        let mut template = Template::from(content);
        template.view = path_buf.parent().unwrap().join(template.view);
        template
    }
}

impl From<&str> for Template {
    fn from(slice: &str) -> Self {
        Template::from(slice.as_bytes())
    }
}

impl From<&[u8]> for Template {
    fn from(slice: &[u8]) -> Self {
        serde_yaml::from_slice(slice).unwrap()
    }
}

impl From<Vec<u8>> for Template {
    fn from(slice: Vec<u8>) -> Self {
        serde_yaml::from_slice(&slice).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Template {
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
    pub pages: HashMap<String, Template>,
    pub components: HashMap<String, Template>,
}

impl Theme {
    pub fn new(base_path: &str) -> Self {
        let pages = parse_templates(&format!("{}{}", base_path, "/pages/*/index.yaml"));
        let components = parse_templates(&format!("{}{}", base_path, "/components/*/index.yaml"));

        Self { pages, components }
    }
}

fn parse_templates(pattern: &str) -> HashMap<String, Template> {
    let mut result = HashMap::new();
    for entry in glob::glob(pattern).expect("Failed to read glob pattern") {
        let path = entry.expect("Failed to read path from glob");
        let template = Template::from(&path);
        let name = get_parent_name(&path).unwrap();
        result.insert(name, template);
    }

    result
}

fn get_parent_name(path: &PathBuf) -> Result<String> {
    let parent = path
        .parent()
        .ok_or(Error::theme("The template has no parent folder."))?;

    let name = parent
        .file_name()
        .ok_or(Error::theme("The parent folder has no name."))?;

    Ok(name.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_template() {
        let template_data = r#"
--- 
view: view.html
fields: 
  field_1: string
areas: 
  areas_1: 
    accept: without
    components: 
      - template_1
  areas_2: 
    accept: with
    components: 
      - template_1
  areas_3: 
    accept: all
        "#;

        let template = Template::from(template_data);

        assert_eq!(template.view, PathBuf::from("view.html"));
        assert_eq!(template.fields.len(), 1);
        assert_eq!(template.areas.len(), 3);
    }
}
