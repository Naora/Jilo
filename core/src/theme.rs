use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

use crate::error::{ThemeError, ThemeErrorKind::*};

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
    pub templates: HashMap<String, Template>,
}

// TODO: Change logic and use something else then OsStr for templates...
impl TryFrom<&PathBuf> for Theme {
    type Error = ThemeError;

    fn try_from(base_path: &PathBuf) -> Result<Self, Self::Error> {
        let mut templates = HashMap::new();

        let canonical = get_canonical(base_path)?;

        let pattern = format!("{}{}", canonical, "/**/index.yaml");
        for entry in glob::glob(&pattern)? {
            let mut path = entry?;
            let template = Template::from(&path);
            path.pop();
            let parent_canonical = get_canonical(&path)?;
            let name = match parent_canonical.split(&canonical).last() {
                Some(str) => str,
                None => return Err(ThemeError::new(ThemeParseError)),
            };
            templates.insert(name.to_string(), template);
        }

        Ok(Self { templates })
    }
}

fn get_canonical(path: &PathBuf) -> Result<String, ThemeError> {
    let canonical = path.canonicalize()?;
    let canonical = match canonical.to_str() {
        Some(str) => str,
        None => return Err(ThemeError::new(ParsePathError)),
    };
    Ok(canonical.to_string())
}

impl From<glob::GlobError> for ThemeError {
    fn from(glob_error: glob::GlobError) -> Self {
        ThemeError::with(ParsePathError, glob_error)
    }
}

impl From<glob::PatternError> for ThemeError {
    fn from(pattern_error: glob::PatternError) -> Self {
        ThemeError::with(ParsePathError, pattern_error)
    }
}

impl From<std::io::Error> for ThemeError {
    fn from(io_error: std::io::Error) -> Self {
        ThemeError::with(CanonicalError, io_error)
    }
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
