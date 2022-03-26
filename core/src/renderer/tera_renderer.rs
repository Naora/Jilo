use crate::{
    error::{Error, Result},
    Module, Renderer, Theme, Value,
};

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
        let context = tera::Context::from(module);

        let name = format!("pages/{}", name);
        self.tera.render(&name, &context).or_else(|error| {
            Err(Error::renderer(format!(
                "Tera could not render page with error: {}",
                error
            )))
        })
    }
}
