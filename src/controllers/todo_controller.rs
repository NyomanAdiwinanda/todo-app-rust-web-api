use crate::{
    database::connection::DbConn,
    models::todo_model::{NewTodo, Todo},
    use_cases::todo_use_case::TodoUseCase,
};
use diesel::SqliteConnection;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};

pub struct TodoController;

impl TodoController {
    pub async fn find_all(db: DbConn) -> (Status, Value) {
        db.run(
            |c: &mut SqliteConnection| match TodoUseCase::find_all(c, 100) {
                Ok(todos) => (
                    Status::Ok,
                    json!({
                        "code": 200,
                        "message": "Success fetch todos",
                        "data": todos,
                    }),
                ),
                Err(_) => (
                    Status::InternalServerError,
                    json!({
                        "code": 500,
                        "message": "Internal server error",
                        "data": null,
                    }),
                ),
            },
        )
        .await
    }

    pub async fn find_one(id: i32, db: DbConn) -> (Status, Value) {
        db.run(
            move |c: &mut SqliteConnection| match TodoUseCase::find_one(c, id) {
                Ok(todo) => (
                    Status::Ok,
                    json!({
                        "code": 200,
                        "message": "Success fetch todo detail",
                        "data": todo,
                    }),
                ),
                Err(_) => (
                    Status::NotFound,
                    json!({
                        "code": 404,
                        "message": "Todo not found",
                        "data": null,
                    }),
                ),
            },
        )
        .await
    }

    pub async fn create(db: DbConn, new_todo: Json<NewTodo>) -> (Status, Value) {
        db.run(move |c: &mut SqliteConnection| {
            match TodoUseCase::create(c, new_todo.into_inner()) {
                Ok(todo) => (
                    Status::Created,
                    json!({
                        "code": 201,
                        "message": "Todo created successfully",
                        "data": todo,
                    }),
                ),
                Err(_) => (
                    Status::InternalServerError,
                    json!({
                        "code": 500,
                        "message": "Internal server error",
                        "data": null,
                    }),
                ),
            }
        })
        .await
    }

    pub async fn update(id: i32, db: DbConn, todo: Json<Todo>) -> (Status, Value) {
        db.run(move |c: &mut SqliteConnection| {
            match TodoUseCase::update(c, id, todo.into_inner()) {
                Ok(todo) => (
                    Status::Ok,
                    json!({
                        "code": 200,
                        "message": "Todo updated successfully",
                        "data": todo,
                    }),
                ),
                Err(_) => (
                    Status::NotFound,
                    json!({
                        "code": 404,
                        "message": "Todo not found",
                        "data": null,
                    }),
                ),
            }
        })
        .await
    }

    pub async fn delete(id: i32, db: DbConn) -> (Status, Value) {
        db.run(
            move |c: &mut SqliteConnection| match TodoUseCase::delete(c, id) {
                Ok(result) => {
                    if result == 1 {
                        (
                            Status::Ok,
                            json!({
                                "code": 200,
                                "message": "Todo deleted successfully",
                                "data": null,
                            }),
                        )
                    } else {
                        (
                            Status::NotFound,
                            json!({
                                "code": 404,
                                "message": "Todo not found",
                                "data": null,
                            }),
                        )
                    }
                }
                Err(_) => (
                    Status::InternalServerError,
                    json!({
                        "code": 500,
                        "message": "Internal server error",
                        "data": null,
                    }),
                ),
            },
        )
        .await
    }
}
