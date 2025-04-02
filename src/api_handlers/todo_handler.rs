use actix_web::{http::StatusCode, web, HttpResponse};
use actix_web::http::header::ContentType;
use crate::api_types::todo_types::Todo;

pub async fn handle_todo(todo: web::Json<Todo>) -> HttpResponse {

    println!("server received item : {}", todo.item);

    let new_todo = Todo {
        item: todo.item.clone(),
    };

    let res = format!("item created : {} ", new_todo.item);
    HttpResponse::Ok().status(StatusCode::OK).content_type(ContentType::json()).body(res)
    // HttpResponse::body(format!("item created : {} ", new_todo.item))
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is working fine!!!")
}
