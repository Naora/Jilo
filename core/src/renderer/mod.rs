use crate::error::Result;
use crate::theme::Theme;
use crate::Module;

pub mod tera_renderer;

pub trait Renderer {
    fn load(&mut self, theme: &Theme) -> Result<()>;
    fn render_page(&self, name: &str, module: &Module) -> Result<String>;
}
