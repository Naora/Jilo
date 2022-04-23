use core::{self, tera_renderer::TeraRenderer, yaml_store::YamlStorage, Site, SiteBuilder, Store};
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
fn render_pages() {
    let mut site = init();

    let result = site.render_page("first_article").unwrap();
    assert!(result.contains("<html lang=\"en\">"));
    assert!(result.contains("<title>A Jilo Title</title>"));
}

#[test]
fn render_pages_with_areas() {
    let mut site = init();

    let result = site.render_page("first_section").unwrap();
    assert!(result.contains("<html lang=\"en\">"));
    assert!(result.contains("<span>Hello Henry</span>"));
}

#[test]
fn create_pages() {
    let storage = YamlStorage::new("./tests/test_site/yaml_storage.yml");
    let pages = storage.summary().unwrap();
    dbg!(pages);
    assert!(false);
}
