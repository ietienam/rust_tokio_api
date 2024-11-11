use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Book {
  pub id: i32,
  pub title: String,
  pub author: String,
}

#[derive(Deserialize)]
pub struct CreateBook {
  pub title: String,
  pub author: String,
}

#[derive(Deserialize)]
pub struct UpdateBook {
  pub title: Option<String>,
  pub author: Option<String>,
}