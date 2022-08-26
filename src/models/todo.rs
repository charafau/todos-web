use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]

pub struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<u32>,
    pub title: String,
}