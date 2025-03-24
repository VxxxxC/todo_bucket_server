use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;
use routes::main_route::init;
use std::env;

mod api_handlers;
mod api_types;
mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = dotenv().ok();
    println!("env : {:?}", env);

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(|| App::new().service(hello).configure(init))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
