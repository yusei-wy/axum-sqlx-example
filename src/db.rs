use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn create_connection_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("Could not read env DATABASE_URL");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to database")
}
