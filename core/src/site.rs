use std::{any::Any, collections::HashMap};

use crate::{
    error::{Error, Result},
    renderer,
    store::Storage,
    theme::Module,
    Context, Field, Renderer, Theme,
};

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
        let pages = self.storage.load_page(&name, &self.theme)?;
        self.renderer.load(&self.theme)?;

        self.renderer.render_page(&name, &Context::new())
    }
}

pub struct Template<'m> {
    module: &'m Module,
    fields: HashMap<String, FieldValue>,
    areas: HashMap<String, Vec<Template<'m>>>,
}

impl<'m> Template<'m> {
    pub fn new(
        module: &'m Module,
        fields: HashMap<String, FieldValue>,
        areas: HashMap<String, Vec<Template<'m>>>,
    ) -> Self {
        Self {
            module,
            fields,
            areas,
        }
    }

    fn set_field_value<S>(&mut self, name: S, value: FieldValue) -> Result<()>
    where
        S: Into<String>,
    {
        let name = name.into();

        if !self.module.fields.contains_key(&name) {
            return Err(Error::site(
                "Tried to add field that does not exists in current module",
            ));
        }

        self.fields.insert(name, value);

        Ok(())
    }

    fn insert_template_to_area<S>(&mut self, area: S, index: usize) -> Result<()>
    where
        S: Into<String>,
    {
        let area = area.into();

        if !self.module.areas.contains_key(&area) {
            return Err(Error::site(
                "Tried to add template in area that does not exists in current module",
            ));
        }

        self.areas.get(&area).unwrap();
        Ok(())
    }
}

pub struct FieldValue {
    r#type: Field,
    value: Box<dyn Any>,
}

impl FieldValue {
    pub(crate) fn new(r#type: Field, value: Box<dyn Any>) -> Self {
        Self { r#type, value }
    }
}
