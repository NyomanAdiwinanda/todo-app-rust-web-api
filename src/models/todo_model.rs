use crate::entities::schema::todos;
use diesel::{
    prelude::{AsChangeset, Insertable},
    Queryable,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, AsChangeset)]
pub struct Todo {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub task: String,
    pub description: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
    #[serde(skip_deserializing)]
    pub updated_at: String,
}

#[derive(Serialize, Queryable)]
pub struct TodosResponse {
    pub id: i32,
    pub task: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub task: String,
    pub description: String,
}
