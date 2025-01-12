use crate::database::connection::DbConn;
use diesel::SqliteConnection;
use rocket::{Build, Rocket};

pub async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATION: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection")
        .run(|c: &mut SqliteConnection| {
            c.run_pending_migrations(MIGRATION)
                .expect("Migration Failed");
        })
        .await;

    rocket
}
