use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]

pub struct Todo {
    pub title: String,
}