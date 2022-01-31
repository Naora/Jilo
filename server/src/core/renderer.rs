use std::{collections::HashMap, path::Path};

use tera::Context;

use super::{
    error::{Error, ErrorKind, Result},
    website::Page,
};

/// Renderer
pub trait Renderer {
    fn render(&self, page: &Page) -> Result<String>;
}

#[derive(Debug)]
pub struct TeraRenderer {
    tera: tera::Tera,
}

impl TeraRenderer {
    pub fn new<P>(views: &HashMap<&String, P>) -> Self
    where
        P: AsRef<Path>,
    {
        let mut tera = tera::Tera::default();

        for (name, view) in views {
            tera.add_template_file(view, Some(name)); // TODO handle error
        }

        TeraRenderer { tera }
    }
}

impl Default for TeraRenderer {
    fn default() -> Self {
        let tera = tera::Tera::default();
        Self { tera }
    }
}

impl Renderer for TeraRenderer {
    fn render(&self, page: &Page) -> Result<String> {
        let mut context = Context::new();
        for (key, val) in &page.data {
            context.insert(key, val);
        }
        match self.tera.render(page.template, &context) {
            Ok(html) => Ok(html),
            Err(e) => Err(Error::with_error(ErrorKind::Renderer, e)),
        }
    }
}
