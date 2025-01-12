use crate::{
    controllers::todo_controller::TodoController, database::connection::DbConn,
    models::todo_model::Todo,
};
use rocket::http::Status;
use rocket::serde::json::{Json, Value};

#[put("/todos/<id>", format = "json", data = "<todo>")]
pub async fn update_todo(id: i32, db: DbConn, todo: Json<Todo>) -> (Status, Value) {
    TodoController::update(id, db, todo).await
}
