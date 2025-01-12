use crate::{
    models::todo_model::{NewTodo, Todo},
    repositories::todo_repository::TodoRepository,
};
use diesel::SqliteConnection;

pub struct TodoUseCase;

impl TodoUseCase {
    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> Result<Vec<Todo>, String> {
        TodoRepository::find_all(c, limit).map_err(|e| e.to_string())
    }

    pub fn find_one(c: &mut SqliteConnection, id: i32) -> Result<Todo, String> {
        TodoRepository::find_one(c, id).map_err(|e| e.to_string())
    }

    pub fn create(c: &mut SqliteConnection, new_todo: NewTodo) -> Result<Todo, String> {
        TodoRepository::create(c, new_todo).map_err(|e| e.to_string())
    }

    pub fn update(c: &mut SqliteConnection, id: i32, todo: Todo) -> Result<Todo, String> {
        TodoRepository::update(c, id, todo).map_err(|e| e.to_string())
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> Result<usize, String> {
        TodoRepository::delete(c, id).map_err(|e| e.to_string())
    }
}
