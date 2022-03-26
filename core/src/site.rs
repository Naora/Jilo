use std::collections::HashMap;

use crate::{error::Result, store::Storage, Renderer, Theme};

pub struct Site<S, R>
where
    S: Storage,
    R: Renderer,
{
    theme: Theme,
    storage: S,
    renderer: R,
}

impl<'t, S, R> Site<S, R>
where
    S: Storage,
    R: Renderer,
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

        self.renderer.render_page(&page.template, &page)
    }
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
