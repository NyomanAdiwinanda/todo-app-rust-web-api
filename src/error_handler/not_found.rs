use rocket::serde::json::{json, Value};

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "code": 404,
        "message": "Not Found",
        "data": null,
    })
}
