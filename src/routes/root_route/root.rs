#[get("/")]
pub fn root() -> &'static str {
    "Welcome to the Todo API!"
}
