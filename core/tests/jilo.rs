use core::{self, Context, Field, Renderer, Site, TeraRenderer, Theme, YamlStorage};

use serde_yaml::Value;

fn init() -> (TeraRenderer, YamlStorage, Theme) {
    let renderer = TeraRenderer::new();
    let storage = YamlStorage::new("./tests/test_site/data");
    let theme = Theme::new("./tests/test_site/theme");
    (renderer, storage, theme)
}

#[test]
fn create_render_page() {
    let (mut renderer, storage, theme) = init();
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
    let (renderer, storage, theme) = init();

    let mut site = Site::new(theme, storage, renderer);
    let result = site.render_page("first_article").unwrap();
    assert!(result.contains("<html lang=\"en\">"));
    assert!(result.contains("<title>A Jilo Title</title>"));
}
