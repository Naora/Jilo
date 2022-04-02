use std::{collections::HashMap, ops::Add};

use tera::{self, Context};

use crate::{
    error::{Error, Result},
    Module, Renderer, Theme, Value,
};

#[derive(Debug)]
pub struct TeraRenderer {
    tera: tera::Tera,
}

impl TeraRenderer {
    fn render_module(&mut self, module: &Module) -> Result<String> {
        self.render_areas(module)
    }

    fn render_areas(&mut self, module: &Module) -> Result<String> {
        let context = Context::from(module);
        for (name, modules) in &module.areas {
            let area_html = String::new();
            for module in modules {
                let partial = self.render_module(module)?;
                area_html = area_html.add(&partial);
            }
            context.insert(name, &area_html);
        }
        let name = format!("components/{}", module.template);
        match self.tera.render(&name, &context) {
            Ok(html) => Ok(html),
            Err(err) => Err(Error::renderer(format!(
                "Could not render area with error {:?}",
                err
            ))),
        }
    }
}

impl From<&Module> for tera::Context {
    fn from(module: &Module) -> Self {
        let mut context = tera::Context::new();
        for (name, value) in &module.fields {
            match value {
                Value::String(val) => context.insert(name, val),
                Value::Integer(val) => context.insert(name, val),
                Value::Boolean(val) => context.insert(name, val),
            };
        }
        context
    }
}

pub struct AreaFunc;

impl tera::Function for AreaFunc {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        println!("{:?}", args);
        Ok(tera::to_value("this is a value").unwrap())
    }
}

impl Default for TeraRenderer {
    fn default() -> Self {
        let mut tera: tera::Tera = Default::default();
        tera.register_function("area", AreaFunc);
        Self { tera }
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
                        "Tera could not load page with error: {}",
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
                        "Tera could not load component with error: {}",
                        error
                    )))
                })?;
        }

        Ok(())
    }

    fn render_page(&self, name: &str, module: &Module) -> Result<String> {
        let html = self.render_module(module)?;

        let name = format!("pages/{}", name);
        self.tera.render(&name, &context).or_else(|error| {
            Err(Error::renderer(format!(
                "Tera could not render page with error: {}",
                error
            )))
        })
    }
}
