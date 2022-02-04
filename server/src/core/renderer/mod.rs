pub mod tera;

use std::{collections::HashMap, path::Path};

use crate::core::{error::Result, website::Page};

/// Renderer
pub trait Renderer {
    fn render(&self, page: &Page) -> Result<String>;
}
