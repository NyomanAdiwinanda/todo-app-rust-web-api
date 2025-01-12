#[macro_use]
extern crate rocket;

mod app;
mod controllers;
mod database;
mod entities;
mod error_handler;
mod models;
mod repositories;
mod routes;
mod use_cases;

#[rocket::main]
async fn main() {
    if let Err(e) = app::server::run_server().await {
        eprintln!("Failed to launch server: {:?}", e);
    }
}
