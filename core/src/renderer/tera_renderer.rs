use std::collections::HashMap;

use tera::{self, Context};

use crate::{
    error::Result,
    module::{Module, Value},
    renderer::Render,
    theme::Theme,
};

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
                Value::Number(val) => context.insert(name, val),
                Value::Boolean(val) => context.insert(name, val),
            };
        }
        context
    }
}

struct AreaFilter;

impl tera::Filter for AreaFilter {
    fn filter(
        &self,
        value: &tera::Value,
        _: &HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        Ok(value.to_owned())
    }

    fn is_safe(&self) -> bool {
        true
    }
}

impl Default for TeraRenderer {
    fn default() -> Self {
        let mut tera: tera::Tera = Default::default();
        tera.register_filter("area", AreaFilter);
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
