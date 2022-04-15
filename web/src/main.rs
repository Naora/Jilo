use actix_files::Files;
use actix_web::{get, middleware::Logger, options, post, web, App, HttpResponse, HttpServer};

use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let public_scope = Files::new("/", "public").index_file("index.html");

        App::new()
            .configure(api_config)
            .service(public_scope)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn api_config(cfg: &mut web::ServiceConfig) {
    let api_v1_scope = web::scope("/api")
        .service(web::scope("/v1"))
        .configure(user_config);
    cfg.service(api_v1_scope);
}

#[derive(Serialize)]
struct Link {
    rel: String,
    href: String,
}

#[derive(Serialize)]
struct Response<T>
where
    T: Serialize,
{
    data: Option<T>,
    error: Option<String>,
    links: Vec<Link>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    fn new(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            links: vec![],
        }
    }

    fn add_link<I>(&mut self, rel: I, href: I)
    where
        I: Into<String>,
    {
        let rel = rel.into();
        let href = href.into();
        self.links.push(Link { rel, href })
    }
}

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
}

#[get("")]
async fn show_all_users() -> HttpResponse {
    let users = vec![User {
        name: "Jojo".to_string(),
        email: "L'asticot".to_string(),
    }];

    let mut response = Response::new(users);
    response.add_link("create user", "/api/v1/users");

    HttpResponse::Ok().json(response)
}

#[options("")]
async fn get_ressources_options() -> HttpResponse {
    let response = stringify!(User);
    HttpResponse::Ok().json(response)
}

#[post("")]
async fn create_new_user() -> HttpResponse {
    HttpResponse::Created().finish()
}

fn user_config(cfg: &mut web::ServiceConfig) {
    let users_scope = web::scope("/users")
        .service(get_ressources_options)
        .service(show_all_users)
        .service(create_new_user);

    cfg.service(users_scope);
}
