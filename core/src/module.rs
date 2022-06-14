use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::theme::Field;
use serde_yaml::{Number, Value};

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

impl From<&Field> for Value {
    fn from(field: &Field) -> Self {
        match *field {
            Field::String => Self::String(String::default()),
            Field::Number => Self::Number(Number::from(usize::default())),
            Field::Boolean => Self::Bool(false),
        }
    }
}
