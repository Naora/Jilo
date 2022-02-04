use std::{collections::HashMap, path::Path};

use crate::core::{
    error::{Error, ErrorKind, Result},
    website::Page,
};

use tera::Context;

use super::Renderer;

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
