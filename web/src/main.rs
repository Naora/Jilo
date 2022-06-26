use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};

use core::SiteBuilder;

mod api;

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
        let public_scope = Files::new("/", "dist/").index_file("index.html");

        App::new()
            .app_data(data.clone())
            .configure(api::config)
            .service(public_scope)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
