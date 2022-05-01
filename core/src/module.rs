use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::theme::Field;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub template: String,
    pub fields: HashMap<String, Value>,
    pub areas: HashMap<String, Vec<Module>>,
}

impl Module {
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
            fields: HashMap::new(),
            areas: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Value {
    String(String),
    Number(usize),
    Boolean(bool),
}

impl From<&Field> for Value {
    fn from(field: &Field) -> Self {
        match *field {
            Field::String => Self::String(String::default()),
            Field::Number => Self::Number(usize::default()),
            Field::Boolean => Self::Boolean(bool::default()),
        }
    }
}
