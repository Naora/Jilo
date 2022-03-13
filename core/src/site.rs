use std::{any::Any, collections::HashMap};

use crate::{
    error::{Error, Result},
    renderer,
    store::Storage,
    theme::Module,
    Field, Renderer, Theme,
};

struct Site<R, S>
where
    R: Renderer,
    S: Storage,
{
    theme: Theme,
    renderer: R,
    store: S,
}

impl<R, S> Site<R, S>
where
    R: Renderer,
    S: Storage,
{
    fn new(theme: Theme, renderer: R, store: S) -> Self {
        Self {
            theme,
            renderer,
            store,
        }
    }
}

struct Template<'m> {
    module: &'m Module,
    fields: HashMap<String, FieldValue>,
    areas: HashMap<String, Vec<Template<'m>>>,
}

impl<'m> Template<'m> {
    fn new(module: &'m Module) -> Self {
        let fields = HashMap::new();
        let areas = HashMap::new();
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

    fn remove_template_from_area() {}
}

// struct Areas {
//     available: Vec<String>,
//     modules: AreaValue,
// }

// struct AreaValue {
//     values: Vec<Template>,
// }

// struct Fields {
//     available: Vec<String>,
//     values: Vec<FieldValue>,
// }

// impl Fields {
//     fn new(map: HashMap<String, Field>) -> Self {
//         let available = vec![];
//         let values = vec![];
//         Self { available, values }
//     }

//     fn set<S>(&mut self, name: S, value: FieldValue) -> Result<()>
//     where
//         S: Into<String>,
//     {
//         Ok(())
//     }

//     fn get<S>(&self, name: S) -> Result<&FieldValue> {
//         Ok(&self.values.get(0).unwrap())
//     }
// }

struct FieldValue {
    r#type: Field,
    value: Box<dyn Any>,
}
