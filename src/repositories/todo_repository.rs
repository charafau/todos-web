use crate::{db::DbError, models::todo_models::*};
use diesel::prelude::*;

pub fn fetch_todos(conn: &PgConnection) -> Result<Option<Vec<Todo>>, DbError> {
    use crate::schema::todos::dsl::*;

    let all_todos = todos.load::<Todo>(conn).optional()?;

    Ok(all_todos)
}

pub fn add_todo(todo_title: &str, conn: &PgConnection) -> Result<Todo, DbError> {
    use crate::schema::todos::dsl::*;

    let new_todo = Todo {
        id: None,
        title: todo_title.to_owned(),
        is_done: false,
    };

    let updated: Todo = diesel::insert_into(todos)
        .values(&new_todo)
        .get_result(conn)?;

    Ok(updated)
}

pub fn fetch_todo(todo_id: &i32, conn: &PgConnection) -> Result<Option<Todo>, DbError> {
    use crate::schema::todos::dsl::*;

    let todo = todos
        .filter(id.eq(todo_id))
        .first::<Todo>(conn)
        .optional()?;

    Ok(todo)
}
