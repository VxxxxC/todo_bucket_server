use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use routes::main_route::init;

mod api_handlers;
mod api_types;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(hello).service(echo).route("hey", web::get().to(manual_hello))
    }).bind("127.0.0.1:8080")?.run().await
}
