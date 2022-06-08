use std::{collections::HashMap, fs, path};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    module::Module,
    store::Store,
};

/// YamlStorage is able to save and load a yaml file as a storage
#[derive(Debug)]
pub struct YamlStorage {
    yaml_file: path::PathBuf,
    storage: YamlFile,
}

impl TryFrom<&str> for YamlStorage {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let file = fs::File::open(value)?;
        let storage = serde_yaml::from_reader(file)?;
        Ok(Self {
            yaml_file: path::PathBuf::from(value),
            storage,
        })
    }
}

impl YamlStorage {
    fn persist_storage(&self) -> Result<()> {
        let writer = fs::File::options().write(true).open(&self.yaml_file)?;
        Ok(serde_yaml::to_writer(writer, &self.storage)?)
    }
}

impl Store for YamlStorage {
    fn summary(&self) -> Vec<String> {
        self.storage.summary()
    }

    fn get_pages(&self) -> Result<HashMap<String, Module>> {
        self.storage.get_pages()
    }

    fn get_page_by_name(&self, name: &String) -> Result<Module> {
        self.storage.get_page_by_name(name)
    }

    fn create_page(&mut self, name: &str, module: Module) -> Result<String> {
        let id = self.storage.create_page(name, module)?;
        self.persist_storage()?;
        Ok(id)
    }

    fn delete_page(&mut self, name: &str) -> Result<Module> {
        let module = self.storage.delete_page(name)?;
        self.persist_storage()?;
        Ok(module)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct YamlFile {
    folder: path::PathBuf,
    pages: HashMap<String, String>,
}

impl YamlFile {
    fn get_uid<T>(&self, generator: &mut T) -> String
    where
        T: IdGenerator,
    {
        let new_id = generator.generate_id();

        match self.pages.iter().find(|(_, value)| *value == &new_id) {
            Some(..) => generator.generate_id(),
            None => new_id,
        }
    }

    fn get_file(&self, id: &str) -> path::PathBuf {
        self.folder.join(format!("{}.yml", &id))
    }
}

impl Store for YamlFile {
    fn summary(&self) -> Vec<String> {
        self.pages
            .iter()
            .map(|(name, ..)| name.to_owned())
            .collect()
    }

    fn get_pages(&self) -> Result<HashMap<String, Module>> {
        let mut pages = HashMap::new();
        for (name, file_name) in &self.pages {
            let file = fs::File::open(self.folder.join(file_name))?;
            let template: Module = serde_yaml::from_reader(file)?;
            pages.insert(name.to_owned(), template);
        }

        Ok(pages)
    }

    fn get_page_by_name(&self, name: &String) -> Result<Module> {
        let id = self.pages.get(name).ok_or(Error::PageNotFound)?;
        let file = fs::File::open(self.get_file(&id))?;
        Ok(serde_yaml::from_reader(file)?)
    }

    fn create_page(&mut self, name: &str, module: Module) -> Result<String> {
        if self.pages.contains_key(name) {
            return Err(Error::DuplicatedName);
        }

        let id = self.get_uid(&mut Random::default());
        let file = fs::File::create(self.get_file(&id))?;
        serde_yaml::to_writer(file, &module)?;
        self.pages.insert(name.to_owned(), id.to_owned());
        Ok(id)
    }

    fn delete_page(&mut self, name: &str) -> Result<Module> {
        if let Some(id) = self.pages.get(name) {
            let path = self.get_file(id);
            let file = fs::File::open(&path)?;
            let module = serde_yaml::from_reader(file)?;
            fs::remove_file(path)?;

            return Ok(module);
        } else {
            Err(Error::PageNotFound)
        }
    }
}

trait IdGenerator {
    fn generate_id(&mut self) -> String;
}

#[derive(Debug, Default)]
struct Random {}

impl IdGenerator for Random {
    fn generate_id(&mut self) -> String {
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    mod id_generator {
        use super::super::*;

        struct SimpleId {
            pub count: usize,
        }

        impl IdGenerator for SimpleId {
            fn generate_id(&mut self) -> String {
                self.count += 1;
                self.count.to_string()
            }
        }

        #[test]
        fn generate_ids() {
            let mut storage = YamlFile {
                folder: "/home".into(),
                pages: HashMap::new(),
            };
            let mut generator = SimpleId { count: 0 };
            let id = storage.get_uid(&mut generator);

            assert_eq!(&id, "1");

            storage.pages.insert("first".to_string(), id);
            generator.count = 0;

            let id = storage.get_uid(&mut generator);

            assert_eq!(&id, "2");
        }
    }
}
