use rocket::serde::json::{json, Value};

#[catch(422)]
pub fn unprocessable_entity() -> Value {
    json!({
        "code": 422,
        "message": "Unprocessable Entity - Validation failed",
        "data": null,
    })
}
