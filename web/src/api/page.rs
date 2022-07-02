use core::Site;

use actix_web::{http::Method, web, HttpResponse};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    let pages = web::resource("/pages")
        .route(web::get().to(show_all_pages))
        .route(web::post().to(create_page))
        .route(web::method(Method::OPTIONS).to(get_pages_options));
    let pages_id = web::resource("/pages/{id}").route(web::method(Method::DELETE).to(delete_page));

    cfg.service(pages);
    cfg.service(pages_id);
}

async fn show_all_pages(site: web::Data<Site>) -> HttpResponse {
    let summary = site.summary();
    HttpResponse::Ok().json(summary)
}

async fn get_pages_options() -> HttpResponse {
    HttpResponse::Ok().json("response")
}

#[derive(Debug, Deserialize)]
struct PageData {
    name: String,
    template: String,
}

async fn create_page(form: web::Json<PageData>, site: web::Data<Site>) -> HttpResponse {
    match site.create_page(&form.name, &form.template) {
        Ok(id) => HttpResponse::Created().json(id),
        Err(error) => match error {
            core::Error::DuplicatedName | core::Error::EmptyPageName => {
                HttpResponse::BadRequest().json(error.to_string())
            }
            _ => HttpResponse::InternalServerError().json(error.to_string()),
        },
    }
}

async fn delete_page(id: web::Path<String>, site: web::Data<Site>) -> HttpResponse {
    match site.delete_page(&id) {
        Ok(..) => HttpResponse::Accepted().finish(),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}
