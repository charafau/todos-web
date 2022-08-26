use actix_web::{Result, get, web};
use crate::models::todo::Todo;

pub fn todo_controller_config(cfg: &mut web::ServiceConfig){
    cfg.service(get_todos);
}

#[get("/todos")]
async fn get_todos() -> Result<String> {
    let todo = Todo {title: String::from("one")};
    Ok(serde_json::to_string(&todo)?)
}
