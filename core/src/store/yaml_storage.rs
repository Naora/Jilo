use std::{collections::HashMap, fs, io::Write, path};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    module::Module,
    store::Store,
};

use super::Page;

/// YamlStorage is able to save and load a yaml file as a storage
#[derive(Debug)]
pub struct YamlStorage {
    yaml_file: path::PathBuf,
    storage: YamlStorageFile,
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
        let mut writer = fs::File::options()
            .write(true)
            .truncate(true)
            .open(&self.yaml_file)?;
        serde_yaml::to_writer(&writer, &self.storage)?;
        Ok(())
    }
}

impl Store for YamlStorage {
    fn summary(&self) -> Vec<Page> {
        self.storage.summary()
    }

    fn get_pages(&self) -> Result<HashMap<String, Module>> {
        self.storage.get_pages()
    }

    fn get_page_by_name(&self, name: &str) -> Option<Module> {
        self.storage.get_page_by_name(name)
    }

    fn page_exists(&self, id: &str) -> bool {
        self.storage.page_exists(id)
    }

    fn create_page(&mut self, name: &str, module: Module) -> Result<String> {
        let id = self.storage.create_page(name, module)?;
        self.persist_storage()?;
        Ok(id)
    }

    fn delete_page(&mut self, id: &str) -> Result<Module> {
        let module = self.storage.delete_page(id)?;
        self.persist_storage()?;
        Ok(module)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct YamlStorageFile {
    folder: path::PathBuf,
    pages: HashMap<String, String>,
}

impl YamlStorageFile {
    fn get_uid<T>(&self, generator: &mut T) -> String
    where
        T: IdGenerator,
    {
        let new_id = generator.generate_id();

        if self.pages.contains_key(&new_id) {
            return generator.generate_id();
        }

        new_id
    }

    fn get_file(&self, id: &str) -> path::PathBuf {
        self.folder.join(format!("{}.yml", &id))
    }

    fn contains_name(&self, page_name: &str) -> Option<(&String, &String)> {
        self.pages.iter().find(|p| p.1 == page_name)
    }
}

impl From<(&String, &String)> for Page {
    fn from(page: (&String, &String)) -> Self {
        Self {
            id: page.0.to_owned(),
            name: page.1.to_owned(),
        }
    }
}

impl Store for YamlStorageFile {
    fn summary(&self) -> Vec<Page> {
        self.pages.iter().map(|p| p.into()).collect()
    }

    fn get_pages(&self) -> Result<HashMap<String, Module>> {
        let mut pages = HashMap::new();
        for (id, name) in &self.pages {
            let file = fs::File::open(self.folder.join(id))?;
            let template: Module = serde_yaml::from_reader(file)?;
            pages.insert(name.to_owned(), template);
        }

        Ok(pages)
    }

    fn get_page_by_name(&self, name: &str) -> Option<Module> {
        if let Some(page) = self.contains_name(name) {
            let file = fs::File::open(self.get_file(&page.0)).ok()?;
            Some(serde_yaml::from_reader(file).ok()?)
        } else {
            None
        }
    }

    fn page_exists(&self, id: &str) -> bool {
        self.pages.contains_key(id)
    }

    fn create_page(&mut self, name: &str, module: Module) -> Result<String> {
        let id = self.get_uid(&mut Random::default());
        let file = fs::File::create(self.get_file(&id))?;
        serde_yaml::to_writer(file, &module)?;
        self.pages.insert(id.to_owned(), name.to_owned());
        Ok(id)
    }

    fn delete_page(&mut self, id: &str) -> Result<Module> {
        let path = self.get_file(id);
        let file = fs::File::open(&path)?;
        let module = serde_yaml::from_reader(file)?;
        fs::remove_file(path)?;
        self.pages.remove(id);
        dbg!(&self.pages);

        Ok(module)
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
            let mut storage = YamlStorageFile {
                folder: "/home".into(),
                pages: HashMap::new(),
            };
            let mut generator = SimpleId { count: 0 };
            let id = storage.get_uid(&mut generator);

            assert_eq!(&id, "1");

            storage.pages.insert(id, "first".to_string());
            generator.count = 0;

            let id = storage.get_uid(&mut generator);

            assert_eq!(&id, "2");
        }
    }
}
