use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    module::Module,
    store::Store,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct YamlStorage {
    folder: PathBuf,
    pages: HashMap<String, String>,
}

impl YamlStorage {
    pub fn new(folder: PathBuf) -> Self {
        Self {
            folder: folder,
            pages: HashMap::new(),
        }
    }

    pub fn from_file<P>(storage_file: P) -> Result<YamlStorage>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(storage_file)?;
        Ok(serde_yaml::from_reader(file)?)
    }

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

    fn get_file(&self, id: &str) -> PathBuf {
        self.folder.join(format!("{}.yml", &id))
    }
}

impl Store for YamlStorage {
    fn summary(&self) -> Result<Vec<String>> {
        todo!()
    }

    fn get_pages(&self) -> Result<HashMap<String, Module>> {
        let mut pages = HashMap::new();
        for (name, file_name) in &self.pages {
            let file = fs::File::open(self.folder.join(file_name))?;
            let template: Module = serde_yaml::from_reader(file)?;
            //let template = page.as_module()?;
            pages.insert(name.to_owned(), template);
        }

        Ok(pages)
    }

    fn get_page_by_name(&self, name: &String) -> Result<Module> {
        let id = self.pages.get(name).ok_or(Error::PageNotFound)?;
        let file = fs::File::open(self.get_file(&id))?;
        Ok(serde_yaml::from_reader(file)?)
    }

    fn create_page(&mut self, name: &str, module: Module) -> Result<()> {
        let id = self.get_uid(&mut Random::default());
        let file = fs::File::create(self.get_file(&id))?;
        serde_yaml::to_writer(file, &module)?;
        self.pages.insert(name.to_owned(), id);
        Ok(())
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
            let mut storage = YamlStorage::new("/home".into());
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
