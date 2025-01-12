use crate::{
    controllers::todo_controller::TodoController, database::connection::DbConn,
    models::todo_model::NewTodo,
};
use rocket::http::Status;
use rocket::serde::json::{Json, Value};

#[post("/todos", format = "json", data = "<new_todo>")]
pub async fn create_todo(db: DbConn, new_todo: Json<NewTodo>) -> (Status, Value) {
    TodoController::create(db, new_todo).await
}
