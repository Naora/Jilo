use core::Site;

use actix_web::{http::Method, web, HttpResponse};
use serde::Deserialize;

use crate::utils::Response;

#[derive(Debug, Deserialize)]
struct PageData {
    name: String,
    template: String,
}

async fn show_all_pages(site: web::Data<Site>) -> HttpResponse {
    let summary = site.summary();
    let response = Response::success(summary);
    HttpResponse::Ok().json(response)
}

async fn get_pages_options() -> HttpResponse {
    HttpResponse::Ok().json("response")
}

async fn create_page(form: web::Json<PageData>, site: web::Data<Site>) -> HttpResponse {
    match site.create_page(&form.name, &form.template) {
        Ok(id) => HttpResponse::Created().json(Response::success(id)),
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
        Err(error) => match error {
            _ => HttpResponse::InternalServerError().json(error.to_string()),
        },
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let pages = web::resource("/pages")
        .route(web::get().to(show_all_pages))
        .route(web::post().to(create_page))
        .route(web::method(Method::OPTIONS).to(get_pages_options));
    let pages_id = web::resource("/pages/{id}").route(web::method(Method::DELETE).to(delete_page));

    cfg.service(pages);
    cfg.service(pages_id);
}
