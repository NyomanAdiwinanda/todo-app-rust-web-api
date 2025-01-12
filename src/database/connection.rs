use rocket_sync_db_pools::database;

#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);
