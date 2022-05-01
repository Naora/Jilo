use actix_web::{http::Method, web, HttpResponse};

async fn show_all_pages() -> HttpResponse {
    HttpResponse::Ok().json("response")
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
