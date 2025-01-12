use crate::{controllers::todo_controller::TodoController, database::connection::DbConn};
use rocket::http::Status;
use rocket::serde::json::Value;

#[get("/todos")]
pub async fn find_all_todo(db: DbConn) -> (Status, Value) {
    TodoController::find_all(db).await
}
