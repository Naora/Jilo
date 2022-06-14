mod error;
mod module;
mod renderer;
mod store;
mod theme;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use module::Module;
use serde::{Deserialize, Serialize};
use store::Page;

use crate::{
    error::Result,
    renderer::{tera_renderer::TeraRenderer, Render},
    store::{yaml_storage::YamlStorage, Store},
    theme::Theme,
};

pub use crate::error::Error;

#[derive(Debug)]
pub struct Site {
    theme: Theme,
    storage: Arc<Mutex<dyn Store>>,
    renderer: Arc<Mutex<dyn Render>>,
}

impl Site {
    pub fn new(
        theme: Theme,
        storage: Arc<Mutex<dyn Store>>,
        renderer: Arc<Mutex<dyn Render>>,
    ) -> Self {
        Self {
            theme,
            storage,
            renderer,
        }
    }

    pub fn create_page(&self, name: &str, template: &str) -> Result<String> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::EmptyPageName);
        }
        let module = self.theme.get_module_defaults(template)?;
        let mut storage_lock = self.storage.lock().unwrap();
        storage_lock.create_page(name, module)
    }

    pub fn delete_page(&self, id: &str) -> Result<Module> {
        let mut storage_lock = self.storage.lock().unwrap();
        storage_lock.delete_page(id)
    }

    pub fn summary(&self) -> Vec<Page> {
        let storage_lock = self.storage.lock().unwrap();
        storage_lock.summary()
    }

    pub fn render_page(&self, name: &str) -> Result<String> {
        let storage_lock = self.storage.lock().unwrap();
        let page = storage_lock.get_page_by_name(&name.to_string())?;
        let mut renderer_lock = self.renderer.lock().unwrap();
        renderer_lock.load(&self.theme)?;

        let html = renderer_lock.render_module(&page)?;
        Ok(html)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Configuration {
    storage: Option<String>,
    renderer: Option<String>,
    #[serde(default = "default_theme")]
    theme: String,
}

fn default_theme() -> String {
    "theme".to_string()
}

pub struct SiteBuilder {
    storage: Option<Arc<Mutex<dyn Store>>>,
    renderer: Option<Arc<Mutex<dyn Render>>>,
    theme: Option<Theme>,
}

impl SiteBuilder {
    pub fn new() -> Self {
        Self {
            storage: None,
            renderer: None,
            theme: None,
        }
    }

    pub fn add_yaml_storage(mut self, path: &str) -> Result<Self> {
        let storage = YamlStorage::try_from(path)?;
        self.storage = Some(Arc::new(Mutex::new(storage)));
        Ok(self)
    }

    pub fn add_tera_renderer(mut self) -> Result<Self> {
        let renderer = TeraRenderer::default();
        self.renderer = Some(Arc::new(Mutex::new(renderer)));
        Ok(self)
    }

    pub fn add_theme(mut self, path: &str) -> Result<Self> {
        self.theme = Some(Theme::from_folder(&PathBuf::from(path))?);
        Ok(self)
    }

    pub fn build(self) -> Site {
        let theme = self
            .theme
            .expect("Could not build site because of missing theme");

        let renderer = self
            .renderer
            .expect("Could not build site because of missing renderer");

        let storage = self
            .storage
            .expect("Could not build site because of missing storage");
        Site::new(theme, Arc::clone(&storage), Arc::clone(&renderer))
    }
}
