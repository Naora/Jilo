use std::collections::HashMap;

use crate::{error::StoreError, site::Module};

pub mod yaml_store;

pub trait Store {
    fn summary(&self) -> Result<Vec<String>, StoreError>;
    fn get_pages(&self) -> Result<HashMap<String, Module>, StoreError>;
    fn get_page_by_name(&self, name: &String) -> Result<Module, StoreError>;
    fn persist(&self, pages: HashMap<String, Module>);
}
