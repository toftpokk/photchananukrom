use std::env;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = Pool::builder().build(manager).unwrap();
    pool
}

// pub fn run_migrations(connection: &mut SqliteConnection) {
//     use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

//     const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

//     connection
//         .run_pending_migrations(MIGRATIONS)
//         .expect("Failed to run migrations");
// }
