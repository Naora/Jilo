use crate::error::RenderError;
use crate::theme::Theme;
use crate::Module;

pub mod tera_renderer;

pub trait Render {
    fn load(&mut self, theme: &Theme) -> Result<(), RenderError>;
    fn render_module(&mut self, module: &Module) -> Result<String, RenderError>;
}
