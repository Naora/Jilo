use std::collections::HashMap;

use crate::{error::Result, site::Module};

pub mod yaml_store;

pub trait Storage {
    fn load(&self) -> Result<HashMap<String, Module>>;
    fn load_page<I>(&self, name: I) -> Result<Module>
    where
        I: Into<String>;

    fn persist(&self, pages: HashMap<String, Module>);
}
