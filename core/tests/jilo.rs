use core::{self, Context, Field, Renderer, TeraRenderer, Theme};

use serde_yaml::Value;

fn init() -> (TeraRenderer, Theme) {
    let mut renderer = TeraRenderer::new();
    let theme = Theme::new("./tests/test_site/theme");
    (renderer, theme)
}

#[test]
fn create_render_page() {
    let (mut renderer, theme) = init();
    let mut context = Context::new();
    context.fields.insert(
        "title".to_string(),
        Value::String("A Jilo Title".to_string()),
    );

    renderer.load(&theme).unwrap();
    let result = renderer.render_page("article", &context).unwrap();
    assert!(result.contains("<html lang=\"en\">"));
    assert!(result.contains("<title>A Jilo Title</title>"));
}

#[test]
fn create_pages() {
    let (renderer, theme) = init();

    let site = Site::new(theme, renderer, "test_site/site_data.yml");
    let pages = site.pages();
    let result = site.render_page("first article");
}
