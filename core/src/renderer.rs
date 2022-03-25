use std::collections::HashMap;

use serde_yaml::Value;
use tera;

use crate::error::{Error, Result};
use crate::theme::Theme;

pub trait Renderer {
    fn load(&mut self, theme: &Theme) -> Result<()>;
    fn render_page(&self, name: &str, context: &Context) -> Result<String>;
}

#[derive(Debug)]
pub struct TeraRenderer {
    tera: tera::Tera,
}

impl TeraRenderer {
    pub fn new() -> Self {
        Self {
            tera: Default::default(),
        }
    }
}

impl From<&Context> for tera::Context {
    fn from(context: &Context) -> Self {
        let mut result = tera::Context::new();
        for (name, value) in &context.fields {
            result.insert(name, &value);
        }
        result
    }
}

impl Renderer for TeraRenderer {
    fn load(&mut self, theme: &Theme) -> Result<()> {
        for (name, page) in &theme.pages {
            let name = format!("pages/{}", name);
            self.tera
                .add_template_file(&page.view, Some(&name))
                .or_else(|error| {
                    Err(Error::renderer(format!(
                        "Tera could not load page with error:\n\n{}",
                        error
                    )))
                })?;
        }

        for (name, component) in &theme.components {
            let name = format!("components/{}", name);
            self.tera
                .add_template_file(&component.view, Some(&name))
                .or_else(|error| {
                    Err(Error::renderer(format!(
                        "Tera could not load component with error:\n\n{}",
                        error
                    )))
                })?;
        }

        Ok(())
    }

    fn render_page(&self, name: &str, context: &Context) -> Result<String> {
        let mut context = tera::Context::from(context);

        let name = format!("pages/{}", name);
        self.tera.render(&name, &context).or_else(|error| {
            Err(Error::renderer(format!(
                "Tera could not render page with error:\n\n{}",
                error
            )))
        })
    }
}

pub struct Context {
    pub fields: HashMap<String, Value>,
    pub areas: HashMap<String, Context>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            areas: HashMap::new(),
        }
    }
}
