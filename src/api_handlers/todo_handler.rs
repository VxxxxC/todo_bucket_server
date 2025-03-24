use actix_web::{web, HttpResponse};
use crate::api_types::todo_types::Todo;

pub async fn handle_todo(todo: web::Json<Todo>) -> HttpResponse {
    let new_todo = Todo {
        item: todo.item.clone(),
    };

    HttpResponse::Created().json(new_todo)
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is working fine!!!")
}
