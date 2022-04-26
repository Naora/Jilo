use std::collections::HashMap;

use tera::{self, Context};

use crate::{error::Result, Module, Render, Theme, Value};

#[derive(Debug)]
pub struct TeraRenderer {
    tera: tera::Tera,
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

struct AreaFunc;

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

impl Render for TeraRenderer {
    fn load(&mut self, theme: &Theme) -> Result<()> {
        for (name, page) in &theme.templates {
            self.tera.add_template_file(&page.view, Some(name))?;
        }

        Ok(())
    }

    fn render_module(&mut self, module: &Module) -> Result<String> {
        let mut context = Context::from(module);
        for (name, modules) in &module.areas {
            let mut area_html = String::new();
            for module in modules {
                let partial = self.render_module(module)?;
                area_html = area_html + &partial;
            }
            context.insert(name, &area_html);
        }

        let html = self.tera.render(&module.template, &context)?;
        Ok(html)
    }
}
