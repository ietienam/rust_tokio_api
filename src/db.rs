use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::config::get_database_url;

pub async fn establish_connection() -> PgPool {
  let database_url = get_database_url();
  PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool")
}