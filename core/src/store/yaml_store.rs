use std::{collections::HashMap, fs, ops::Add, path};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    Module, Storage, Value,
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

    fn get_all_pages(&self) -> Vec<YamlPage> {
        let mut data = vec![];
        let pattern = self.base_path.clone().add("/**/*.yml");
        for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
            let entry = entry.expect("Could not load data from directory {}");
            println!("{:?}", &entry);
            let page = self.load_file(entry).expect("Could not retrieve file");
            data.push(page);
        }
        data
    }

    fn load_file<P>(&self, path: P) -> Result<YamlPage>
    where
        P: AsRef<path::Path>,
    {
        println!("{:?}", path.as_ref());
        let file = fs::File::open(path).or_else(|error| {
            Err(Error::store(format!(
                "Could not open file with error {:?}",
                error
            )))
        })?;

        let data = serde_yaml::from_reader(file).or_else(|error| {
            Err(Error::store(format!(
                "Could not deserialize data with error {:?}",
                error
            )))
        })?;

        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct YamlPage {
    name: String,
    template: String,
    fields: serde_yaml::Mapping,
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

    fn as_module(&self) -> Result<Module> {
        let mut module = Module::new(&self.template);
        module.fields = self.get_fields()?;
        Ok(module)
    }
}

impl Storage for YamlStorage {
    fn load(&self) -> Result<HashMap<String, Module>> {
        let data = self.get_all_pages();

        let mut pages = HashMap::new();
        for page in data {
            let template = page.as_module()?;
            pages.insert(page.name.clone(), template);
        }

        Ok(pages)
    }

    fn persist(&self, _: HashMap<String, Module>) {
        todo!()
    }

    fn load_page<I>(&self, name: I) -> Result<Module>
    where
        I: Into<String>,
    {
        let name = name.into();
        let file = format!("/{}.yml", name);
        let path = self.base_path.clone().add(&file);
        let page_data = self.load_file(path)?;

        page_data.as_module()
    }
}

impl TryFrom<&serde_yaml::Value> for Value {
    type Error = Error;

    fn try_from(value: &serde_yaml::Value) -> std::result::Result<Self, Error> {
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
