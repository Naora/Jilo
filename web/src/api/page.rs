use actix_web::{http::Method, web, HttpResponse};
use serde::Deserialize;

use crate::utils::Response;

use super::AppState;

#[derive(Debug, Deserialize)]
struct PageData {
    name: String,
    template: String,
}

async fn show_all_pages(state: web::Data<AppState>) -> HttpResponse {
    let summary = state.site.summary();
    let response = Response::success(summary);
    HttpResponse::Ok().json(response)
}

async fn get_pages_options() -> HttpResponse {
    HttpResponse::Ok().json("response")
}

async fn create_page(form: web::Form<PageData>, state: web::Data<AppState>) -> HttpResponse {
    match state.site.create_page(&form.name, &form.template) {
        Ok(..) => HttpResponse::Created().finish(),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let users_scope = web::resource("/pages")
        .route(web::get().to(show_all_pages))
        .route(web::post().to(create_page))
        .route(web::method(Method::OPTIONS).to(get_pages_options));

    cfg.service(users_scope);
}
