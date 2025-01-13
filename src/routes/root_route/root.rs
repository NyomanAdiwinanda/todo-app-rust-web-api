#[get("/")]
pub fn root() -> &'static str {
    "Welcome to the Todo API!"
}

#[head("/")]
pub fn root_head() -> &'static str {
    ""
}
