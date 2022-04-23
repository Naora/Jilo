use actix_web::{http::Method, web, HttpResponse};

use crate::utils::Response;

async fn show_all_pages(data: web::Data<super::AppState>) -> HttpResponse {
    let response = match data.storage.summary() {
        Ok(summary) => Response::success(summary),
        Err(error) => Response::error(error.to_string()),
    };

    HttpResponse::Ok().json(response)
}

async fn get_pages_options() -> HttpResponse {
    HttpResponse::Ok().json("response")
}

async fn create_new_user() -> HttpResponse {
    HttpResponse::Created().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let users_scope = web::resource("/pages")
        .route(web::get().to(show_all_pages))
        .route(web::post().to(create_new_user))
        .route(web::method(Method::OPTIONS).to(get_pages_options));

    cfg.service(users_scope);
}
