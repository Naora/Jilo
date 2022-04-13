use core::{self, tera_renderer::TeraRenderer, yaml_store::YamlStorage, Site, SiteBuilder};
use std::path::PathBuf;

fn init() -> Site<YamlStorage, TeraRenderer> {
    let renderer = TeraRenderer::default();
    let storage = YamlStorage::new("./tests/test_site/data");

    SiteBuilder::new()
        .theme(&PathBuf::from("./tests/test_site/theme"))
        .renderer(renderer)
        .storage(storage)
        .build()
}

#[test]
fn create_pages() {
    let mut site = init();

    let result = site.render_page("first_article").unwrap();
    assert!(result.contains("<html lang=\"en\">"));
    assert!(result.contains("<title>A Jilo Title</title>"));
}

#[test]
fn create_pages_with_areas() {
    let mut site = init();

    let result = site.render_page("first_section").unwrap();
    assert!(result.contains("<html lang=\"en\">"));
    assert!(result.contains("<span>Hello Henry</span>"));
}
