use crate::{controllers::todo_controller::TodoController, database::connection::DbConn};
use rocket::http::Status;
use rocket::serde::json::Value;

#[get("/todos/<id>")]
pub async fn find_one_todo(id: i32, db: DbConn) -> (Status, Value) {
    TodoController::find_one(id, db).await
}
