use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{error::Result, store::Store, Render, Theme};

pub struct Site<S, R>
where
    S: Store,
    R: Render,
{
    theme: Theme,
    storage: S,
    renderer: R,
}

impl<S, R> Site<S, R>
where
    S: Store,
    R: Render,
{
    pub fn new(theme: Theme, storage: S, renderer: R) -> Self {
        Self {
            theme,
            storage,
            renderer,
        }
    }

    pub fn render_page<I>(&mut self, name: I) -> Result<String>
    where
        I: Into<String>,
    {
        let name = name.into();
        let page = self.storage.load_page(&name)?;
        self.renderer.load(&self.theme)?;

        self.renderer.render_module(&page)
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

#[derive(Debug)]
pub struct Module {
    pub template: String,
    pub fields: HashMap<String, Value>,
    pub areas: HashMap<String, Vec<Module>>,
}

impl Module {
    pub fn new<I>(template: I) -> Self
    where
        I: Into<String>,
    {
        Self {
            template: template.into(),
            fields: HashMap::new(),
            areas: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Integer(usize),
    Boolean(bool),
}

pub struct SiteBuilder<R, S>
where
    R: Render,
    S: Store,
{
    storage: Option<S>,
    renderer: Option<R>,
    theme: Option<Theme>,
}

impl<R, S> SiteBuilder<R, S>
where
    R: Render,
    S: Store,
{
    pub fn new() -> Self {
        Self {
            storage: None,
            renderer: None,
            theme: None,
        }
    }

    pub fn storage(mut self, storage: S) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn renderer(mut self, render: R) -> Self {
        self.renderer = Some(render);
        self
    }

    pub fn theme(mut self, base_path: &PathBuf) -> Self {
        self.theme = Some(Theme::try_from(base_path).unwrap());
        self
    }

    pub fn build(self) -> Site<S, R> {
        let theme = self
            .theme
            .expect("Could not build site because of missing theme");

        let renderer = self
            .renderer
            .expect("Could not build site because of missing renderer");

        let storage = self
            .storage
            .expect("Could not build site because of missing storage");
        Site::new(theme, storage, renderer)
    }
}
