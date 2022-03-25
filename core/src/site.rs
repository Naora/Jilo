use std::{any::Any, collections::HashMap};

use crate::{error::Result, store::Storage, Context, Field, Renderer, Theme};

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

        self.renderer.render_page(&page.template, &Context::new())
    }
}

pub struct Module {
    template: String,
    fields: HashMap<String, FieldValue>,
    areas: HashMap<String, Vec<Module>>,
}

impl Module {
    pub fn new(
        template: String,
        fields: HashMap<String, FieldValue>,
        areas: HashMap<String, Vec<Module>>,
    ) -> Self {
        Self {
            template,
            fields,
            areas,
        }
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
