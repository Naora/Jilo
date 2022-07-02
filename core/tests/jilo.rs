use core::{self, SiteBuilder};

#[test]
fn create_page() {
    let site = SiteBuilder::new()
        .add_tera_renderer()
        .unwrap()
        .add_yaml_storage("./tests/test_site/yaml_storage.yml")
        .unwrap()
        .add_theme("./tests/test_site/theme")
        .unwrap()
        .build();

    let id = site.create_page("first test", "/pages/section").unwrap();
    let summary = site.summary();

    assert_eq!(summary.len(), 1);
    // assert_eq!(summary.get(&id).unwrap(), "first test");

    site.delete_page(&id).unwrap();
}
