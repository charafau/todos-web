use actix_web::{Result, get, post, web, HttpResponse, Error};
use crate::models::todo::Todo;
use crate::db::DbPool;
use crate::repositories::*;

pub fn todo_controller_config(cfg: &mut web::ServiceConfig){
    cfg.service(get_todos);
    cfg.service(post_todo);
}

#[get("/todos")]
async fn get_todos(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");
    let all_todos = todo_repository::fetch_todos(&conn).map_err(actix_web::error::ErrorInternalServerError)?;


    if let Some(all_todos) = all_todos {
        Ok(HttpResponse::Ok().json(all_todos))
    } else {
        let empty: Vec<Todo> = vec![];
        Ok(HttpResponse::Ok().json(empty))
    }
   
}

#[post("/todos")]
async fn post_todo(todo: web::Json<Todo>) -> Result<String> {
    Ok(serde_json::to_string(&todo)?)
}