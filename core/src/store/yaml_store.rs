use std::{
    collections::HashMap,
    fs,
    path::{self},
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    Module, Store, Value,
};

pub struct YamlStorage {
    base_path: String,
}

impl YamlStorage {
    pub fn new<I>(base_path: I) -> Self
    where
        I: Into<String>,
    {
        let base_path = base_path.into();
        Self { base_path }
    }

    fn get_all_pages(&self) -> HashMap<String, YamlPage> {
        let mut data = HashMap::new();
        let pattern = format!("{}{}", self.base_path, "/**/*.yml");
        for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
            let entry = entry.expect("Could not load data from directory {}");
            let page = self.load_file(&entry).expect("Could not retrieve file");
            let file_name = entry.file_name().unwrap();
            let file_name = file_name.to_str().unwrap();
            let extention = entry.extension().unwrap();
            let extention = extention.to_str().unwrap();
            let name = file_name.trim_end_matches(extention);
            data.insert(name.to_string(), page);
        }
        data
    }

    fn load_file<P>(&self, path: P) -> Result<YamlPage>
    where
        P: AsRef<path::Path>,
    {
        let file = fs::File::open(path)?;
        let data = serde_yaml::from_reader(file)?;

        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct YamlPage {
    template: String,
    fields: serde_yaml::Mapping,
    areas: Option<serde_yaml::Mapping>,
}

impl YamlPage {
    fn get_fields(&self) -> Result<HashMap<String, Value>> {
        let mut fields = HashMap::new();
        for (key, value) in &self.fields {
            let name = key.as_str().unwrap().to_owned();
            let field_value = Value::try_from(value)?;
            fields.insert(name, field_value);
        }
        Ok(fields)
    }

    fn get_areas(&self) -> Result<HashMap<String, Vec<Module>>> {
        let mut areas = HashMap::new();
        if let Some(mappings) = &self.areas {
            for (key, value) in mappings {
                if let serde_yaml::Value::Sequence(sequence) = value {
                    let name = key.as_str().unwrap().to_owned();
                    let mut modules = vec![];
                    for value in sequence {
                        let areas = YamlPage::try_from(value)?;
                        modules.push(areas.as_module()?);
                    }
                    areas.insert(name, modules);
                } else {
                    let message = format!(
                        "Expected to find a yaml sequence in areas. But found {:?}",
                        value
                    );
                    return Err(Error::store(message));
                }
            }
        }
        Ok(areas)
    }

    fn as_module(&self) -> Result<Module> {
        let mut module = Module::new(&self.template);
        module.fields = self.get_fields()?;
        module.areas = self.get_areas()?;
        Ok(module)
    }
}

impl Store for YamlStorage {
    fn summary(&self) -> Result<Vec<String>> {
        todo!()
    }

    fn load(&self) -> Result<HashMap<String, Module>> {
        let data = self.get_all_pages();

        let mut pages = HashMap::new();
        for (name, page) in data {
            let template = page.as_module()?;
            pages.insert(name, template);
        }

        Ok(pages)
    }

    fn load_page(&self, name: &String) -> Result<Module> {
        let path = format!("{}/{}.yml", self.base_path, name);
        let page_data = self.load_file(path)?;

        page_data.as_module()
    }

    fn persist(&self, _: HashMap<String, Module>) {
        todo!()
    }
}

impl TryFrom<&serde_yaml::Value> for Value {
    type Error = Error;

    fn try_from(value: &serde_yaml::Value) -> Result<Self> {
        let field_value = match value {
            serde_yaml::Value::Bool(val) => Value::Boolean(val.clone()),
            serde_yaml::Value::Number(val) => {
                let value = val.as_i64().unwrap().try_into().unwrap();
                Value::Integer(value)
            }
            serde_yaml::Value::String(val) => Value::String(val.clone()),
            _ => return Err(Error::store("a null value cannot be loaded")),
        };

        Ok(field_value)
    }
}

impl TryFrom<&serde_yaml::Value> for YamlPage {
    type Error = Error;

    fn try_from(value: &serde_yaml::Value) -> Result<Self> {
        let module: YamlPage = serde_yaml::from_value(value.to_owned()).unwrap();

        Ok(module)
    }
}
