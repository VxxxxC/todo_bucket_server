use crate::api_handlers::todo_handler::{handle_todo, health_check};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("todo", web::post().to(handle_todo))
            .route("/health", web::get().to(health_check)),
    );
}
