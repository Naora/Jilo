mod core;
mod utils;

// use crate::core::Renderer;

// use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello(app: web::Data<Application>) -> impl Responder {
//     let mut data = tera::Context::new();
//     data.insert("product_name", &"This is a product in index".to_string());
//     let html = &app.render("product", &data);
//     HttpResponse::Ok().body(html)
// }

// async fn manual_hello(app: web::Data<Application>) -> impl Responder {
//     let mut data = tera::Context::new();
//     data.insert("content", &"Hello There".to_string());
//     let html = &app.render("base", &data);
//     HttpResponse::Ok().body(html)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let core = core::Core::default();

    // let mut data = tera::Context::new();
    // data.insert("product_name", &"This is a product in index".to_string());
    // let html = core.renderer.render("product", &data).unwrap();

    // HttpServer::new(|| {
    //     App::new()
    //         .data(Application::default())
    //         .service(hello)
    //         .wrap(Logger::default())
    //         .wrap(Logger::new("%a %{User-Agent}i"))
    //         .route("/hey", web::get().to(manual_hello))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await

    Ok(())
}
