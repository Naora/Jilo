use std::{
    any::Any,
    collections::HashMap,
    fs,
    io::Read,
    ops::Add,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

use crate::{
    error::{Error, Result},
    site::Template,
    Field, FieldValue, Theme,
};

pub trait Storage {
    fn load<'t>(&self, theme: &'t Theme) -> Result<HashMap<String, Template<'t>>>;
    fn load_page<'t, I>(&self, name: I, theme: &'t Theme) -> Result<Template<'t>>
    where
        I: Into<String>;

    fn persist(&self, pages: HashMap<String, Template>);
}

pub struct YamlStorage {
    pattern: String,
}

impl YamlStorage {
    pub fn new<I>(base_path: I) -> Self
    where
        I: Into<String>,
    {
        let path = base_path.into();
        let pattern = path.add("/**/*.yml");
        Self { pattern }
    }

    fn get_all_pages(&self) -> Vec<YamlPage> {
        let mut data = vec![];
        for entry in glob::glob(&self.pattern).expect("Failed to read glob pattern") {
            let entry = entry.expect("Could not load data from directory {}");
            let page = self.get_page(entry).expect("Could not retrieve file");
            data.push(page);
        }
        data
    }

    fn get_page<P>(&self, path: P) -> Result<YamlPage>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(path).or_else(|error| {
            Err(Error::store(format!(
                "Could not open file with error \n{:?}",
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
    module: String,
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

    fn as_template<'t>(&self, theme: &'t Theme) -> Result<Template<'t>> {
        let module = theme
            .pages
            .get(&self.module)
            .ok_or(Error::store("The module could not be found in the Theme."))?;

        let fields = self.get_fields()?;
        Ok(Template::new(module, fields, HashMap::new()))
    }
}

impl Storage for YamlStorage {
    fn load<'t>(&self, theme: &'t Theme) -> Result<HashMap<String, Template<'t>>> {
        let data = self.get_all_pages();

        let mut pages = HashMap::new();
        for page in data {
            let template = page.as_template(theme)?;
            pages.insert(page.name.clone(), template);
        }

        Ok(pages)
    }

    fn persist(&self, pages: HashMap<String, Template>) {
        todo!()
    }

    fn load_page<'t, I>(&self, name: I, theme: &'t Theme) -> Result<Template<'t>>
    where
        I: Into<String>,
    {
        let name = name.into();
        let page_data = self.get_page(&name)?;

        page_data.as_template(theme)
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
