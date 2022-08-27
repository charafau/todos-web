use crate::db::DbPool;
use crate::models::todo_models::{NewTodo, Todo};
use crate::repositories::*;
use actix_web::{get, post, web, Error, HttpResponse, Result};

pub fn todo_controller_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_todos);
    cfg.service(post_todo);
    cfg.service(get_todo);
}

#[get("/todos")]
async fn get_todos(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let all_todos =
        todo_repository::fetch_todos(&conn).map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(all_todos) = all_todos {
        Ok(HttpResponse::Ok().json(all_todos))
    } else {
        let empty: Vec<Todo> = vec![];
        Ok(HttpResponse::Ok().json(empty))
    }
}

#[post("/todos")]
async fn post_todo(
    pool: web::Data<DbPool>,
    form: web::Json<NewTodo>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let updated_todo = todo_repository::add_todo(&form.title, &conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updated_todo))
}

#[get("/todos/{todo_id}")]
async fn get_todo(pool: web::Data<DbPool>, todo_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let todo = todo_repository::fetch_todo(&todo_id, &conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(todo))
}
