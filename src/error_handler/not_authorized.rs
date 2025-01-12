use rocket::serde::json::{json, Value};

#[catch(401)]
pub fn not_authorized() -> Value {
    json!({
        "code": 401,
        "message": "Not Authorized",
        "data": null,
    })
}
