mod api;
mod utils;

use actix_files::{Files, NamedFile};
use actix_web::{get, middleware::Logger, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let public_scope = Files::new("/assets", "public/assets/");

        App::new()
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
