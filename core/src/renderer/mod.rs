use crate::error::Result;
use crate::theme::Theme;
use crate::Module;

pub mod tera_renderer;

pub trait Render {
    fn load(&mut self, theme: &Theme) -> Result<()>;
    fn render_module(&mut self, module: &Module) -> Result<String>;
}
