use std::collections::HashMap;

use crate::{error::Result, site::Module};

pub mod yaml_store;

pub trait Store {
    fn summary(&self) -> Result<Vec<String>>;
    fn load(&self) -> Result<HashMap<String, Module>>;
    fn load_page(&self, name: &String) -> Result<Module>;
    fn persist(&self, pages: HashMap<String, Module>);
}
