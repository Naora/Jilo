use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

use crate::{error::Result, module::Module};

pub mod yaml_storage;

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub name: String,
}

pub trait Store: fmt::Debug + Send + Sync {
    fn summary(&self) -> Vec<Page>;
    fn get_pages(&self) -> Result<HashMap<String, Module>>;
    fn get_page_by_name(&self, name: &String) -> Result<Module>;
    fn create_page(&mut self, name: &str, module: Module) -> Result<String>;
    fn delete_page(&mut self, name: &str) -> Result<Module>;
}
