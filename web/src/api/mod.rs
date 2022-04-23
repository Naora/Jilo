mod page;

use actix_web::web;

struct AppState {
    renderer: Box<dyn core::Render>,
    storage: Box<dyn core::Store>,
}

impl Default for AppState {
    fn default() -> Self {
        let storage = core::yaml_store::YamlStorage::new("./core/tests/test_site/data");
        let renderer = core::tera_renderer::TeraRenderer::default();
        Self {
            renderer: Box::new(renderer),
            storage: Box::new(storage),
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let data = web::Data::new(AppState::default());

    let api_v1_scope = web::scope("/api")
        .service(web::scope("/v1"))
        .app_data(data.clone())
        .configure(page::config);

    cfg.service(api_v1_scope);
}
