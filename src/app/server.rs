use crate::{
    database::connection::DbConn,
    database::run_db_migrations::*,
    error_handler::{
        bad_request::*, not_authorized::*, not_found::*, server_error::*, unprocessable_entity::*,
    },
    routes::todo_route::{
        create_todo::*, delete_todo::*, find_all_todo::*, find_one_todo::*, update_todo::*,
    },
};
use rocket::fairing::AdHoc;

pub async fn run_server() -> Result<(), rocket::Error> {
    rocket::build()
        .mount(
            "/",
            routes![
                find_all_todo,
                find_one_todo,
                create_todo,
                update_todo,
                delete_todo
            ],
        )
        .register(
            "/",
            catchers![
                bad_request,
                not_authorized,
                not_found,
                unprocessable_entity,
                server_error
            ],
        )
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await?;

    Ok(())
}
