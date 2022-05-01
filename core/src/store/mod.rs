use std::{collections::HashMap, fmt};

use crate::{error::Result, module::Module};

pub mod yaml_store;

pub trait Store: fmt::Debug {
    fn summary(&self) -> Result<Vec<String>>;
    fn get_pages(&self) -> Result<HashMap<String, Module>>;
    fn get_page_by_name(&self, name: &String) -> Result<Module>;
    fn create_page(&mut self, name: &str, module: Module) -> Result<()>;
}
