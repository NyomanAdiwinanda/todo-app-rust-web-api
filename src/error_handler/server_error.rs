use rocket::serde::json::{json, Value};

#[catch(500)]
pub fn server_error() -> Value {
    json!({
        "code": 500,
        "message": "Internal server error",
        "data": null,
    })
}
