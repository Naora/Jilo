mod api;
mod utils;

use actix_files::{Files, NamedFile};
use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};

use core::SiteBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let site = SiteBuilder::new()
        .add_tera_renderer()
        .unwrap()
        .add_yaml_storage("./core/tests/test_site/yaml_storage.yml")
        .unwrap()
        .add_theme("./core/tests/test_site/theme")
        .unwrap()
        .build();

    let data = web::Data::new(site);

    HttpServer::new(move || {
        let public_scope = Files::new("/assets", "public/assets/");

        App::new()
            .app_data(data.clone())
            .configure(api::config)
            .service(public_scope)
            .service(index)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/{route:.*}")]
async fn index() -> impl Responder {
    NamedFile::open_async("public/index.html").await
}
