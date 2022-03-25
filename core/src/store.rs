use std::{any::Any, collections::HashMap, fs, ops::Add, path::Path};

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

use crate::{
    error::{Error, Result},
    site::Module,
    Field, FieldValue,
};

pub trait Storage {
    fn load(&self) -> Result<HashMap<String, Module>>;
    fn load_page<I>(&self, name: I) -> Result<Module>
    where
        I: Into<String>;

    fn persist(&self, pages: HashMap<String, Module>);
}

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
        P: AsRef<Path>,
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
    fields: Mapping,
}

impl YamlPage {
    fn get_fields(&self) -> Result<HashMap<String, FieldValue>> {
        let mut fields = HashMap::new();
        for (key, value) in &self.fields {
            let name = key.as_str().unwrap().to_owned();
            let field_value = FieldValue::try_from(value)?;
            fields.insert(name, field_value);
        }
        Ok(fields)
    }

    fn as_module(&self) -> Result<Module> {
        let template = self.template.clone();
        let fields = self.get_fields()?;
        Ok(Module::new(template, fields, HashMap::new()))
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

    fn persist(&self, pages: HashMap<String, Module>) {
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

impl TryFrom<&Value> for FieldValue {
    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Error> {
        let (r#type, value): (Field, Box<dyn Any>) = match value {
            Value::Bool(val) => (Field::Boolean, Box::new(val.to_owned())),
            Value::Number(val) => (Field::Integer, Box::new(val.as_i64().unwrap())),
            Value::String(val) => (Field::String, Box::new(val.to_owned())),
            _ => return Err(Error::store("a null value cannot be loaded")),
        };

        Ok(Self::new(r#type, value))
    }
}
