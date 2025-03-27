use actix_web::{http::StatusCode, web, HttpResponse};
use crate::api_types::todo_types::Todo;

pub async fn handle_todo(todo: web::Json<Todo>) -> HttpResponse {
    let new_todo = Todo {
        item: todo.item.clone(),
    };

    let res = format!("item created : {} ", new_todo.item);
    HttpResponse::Ok().status(StatusCode::OK).body(res)
    // HttpResponse::body(format!("item created : {} ", new_todo.item))
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is working fine!!!")
}
