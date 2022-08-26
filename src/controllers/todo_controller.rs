use actix_web::{Result, get, post, web};
use crate::models::todo::Todo;

pub fn todo_controller_config(cfg: &mut web::ServiceConfig){
    cfg.service(get_todos);
    cfg.service(post_todo);
}

#[get("/todos")]
async fn get_todos() -> Result<String> {
    let todo = Todo {id: None, title: String::from("one")};
    Ok(serde_json::to_string(&todo)?)
}

#[post("/todos")]
async fn post_todo(todo: web::Json<Todo>) -> Result<String> {
    Ok(serde_json::to_string(&todo)?)
}