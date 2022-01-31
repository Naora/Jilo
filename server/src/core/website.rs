use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::module::Module;

pub struct Page<'a> {
    pub template: &'a str,
    pub data: HashMap<String, String>,
    pub modules: Vec<String>,
}

#[derive(Debug)]
pub struct Website {
    pages: HashMap<String, Module>,
    modules: HashMap<String, Module>,
}

impl Website {
    pub fn new(base_path: &str) -> Self {
        let mut pages = HashMap::new();
        let mut modules = HashMap::new();

        if Path::new(base_path).is_dir() {
            pages.parse_module(&format!("{}{}", base_path, "/pages/**/index.toml"));
            modules.parse_module(&format!("{}{}", base_path, "/modules/**/index.toml"));
        }
        Self { pages, modules }
    }

    pub fn get_views(&self) -> HashMap<&String, &PathBuf> {
        let mut result = HashMap::new();

        for (name, page) in &self.pages {
            if let Some(view) = &page.view {
                result.insert(name, view);
            }
        }

        for (name, module) in &self.modules {
            if let Some(view) = &module.view {
                result.insert(name, view);
            }
        }

        result
    }
}

trait ParseModuleForPattern {
    fn parse_module(&mut self, pattern: &str) -> ();
}

impl ParseModuleForPattern for HashMap<String, Module> {
    fn parse_module(&mut self, pattern: &str) -> () {
        for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(mut path) => {
                    let module = Module::new(&path);
                    path.pop();
                    match module.get_module_name() {
                        Ok(name) => {
                            self.insert(name, module);
                        }
                        Err(error) => {
                            log::warn!("Could not add module - {}", error)
                        }
                    };
                }
                _ => (),
            }
        }
    }
}
