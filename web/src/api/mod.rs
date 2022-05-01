mod page;

use core::{Site, SiteBuilder};

use actix_web::web;

struct AppState {
    site: Site,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let site = SiteBuilder::new()
        .add_tera_renderer()
        .unwrap()
        .add_yaml_storage("./core/tests/test_site/data")
        .unwrap()
        .add_theme("./core/tests/test_site/theme")
        .unwrap()
        .build();

    let app_state = AppState { site };

    let data = web::Data::new(app_state);

    let api_v1_scope = web::scope("/api")
        .service(web::scope("/v1"))
        .app_data(data.clone())
        .configure(page::config);

    cfg.service(api_v1_scope);
}
