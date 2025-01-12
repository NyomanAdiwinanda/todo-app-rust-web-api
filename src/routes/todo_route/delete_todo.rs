use crate::{controllers::todo_controller::TodoController, database::connection::DbConn};
use rocket::http::Status;
use rocket::serde::json::Value;

#[delete("/todos/<id>")]
pub async fn delete_todo(id: i32, db: DbConn) -> (Status, Value) {
    TodoController::delete(id, db).await
}
