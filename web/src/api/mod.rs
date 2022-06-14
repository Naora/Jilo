mod page;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    let v1 = web::scope("/v1").configure(page::config);

    let api_v1_scope = web::scope("/api").service(v1);

    cfg.service(api_v1_scope);
}
