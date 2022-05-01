use core::{self, SiteBuilder};

#[test]
fn create_page() {
    let mut site = SiteBuilder::new()
        .add_tera_renderer()
        .unwrap()
        .add_yaml_storage("./tests/test_site/data")
        .unwrap()
        .add_theme("./tests/test_site/theme")
        .unwrap()
        .build();

    site.create_page("first section", "/pages/section").unwrap();
    let summary = site.summary();

    assert_eq!(summary.len(), 1);
    assert_eq!(&summary[0], "first section");

    site.delete_page("first section").unwrap();
}
