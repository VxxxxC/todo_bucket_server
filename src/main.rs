use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use dotenv::dotenv;
use routes::main_route::init;
use std::{env, time::Duration};

mod api_handlers;
mod api_types;
mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/exit")]
async fn exit() -> HttpResponse {
    println!("Shutting down the server...");
    
    // Shuts down server after 2 seconds
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        std::process::exit(0);
    });

    // Return a shutdown message to the client
    HttpResponse::Ok().body("Server is shutting down...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = dotenv().ok();
    println!("env : {:?}", env);

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(|| App::new().service(hello).service(exit).configure(init))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
