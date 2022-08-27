use serde::{Deserialize, Serialize};

use crate::schema::todos;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]

pub struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<i32>,
    pub title: String,
    pub is_done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTodo {
    pub title: String,
}