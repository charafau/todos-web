use crate::{db::DbError, models::todo::Todo};
use diesel::prelude::*;

pub fn fetch_todos(conn: &PgConnection) -> Result<Option<Vec<Todo>>, DbError> {
    use crate::schema::todos::dsl::*;

    let todo = todos.load::<Todo>(conn).optional()?;

    Ok(todo)
}
