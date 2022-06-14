use std::fmt;

use crate::{error::Result, module::Module, theme::Theme};

pub mod tera_renderer;

pub trait Render: fmt::Debug + Send + Sync {
    fn load(&mut self, theme: &Theme) -> Result<()>;
    fn render_module(&mut self, module: &Module) -> Result<String>;
}
