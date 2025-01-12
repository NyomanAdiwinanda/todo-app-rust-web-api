use rocket::serde::json::{json, Value};

#[catch(400)]
pub fn bad_request() -> Value {
    json!({
        "code": 400,
        "message": "Bad request",
        "data": null,
    })
}
