use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("Database error")]
  DatabaseError(#[from] sqlx::Error),
  #[error("Not found")]
  NotFound,
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    match self {
      ApiError::DatabaseError(_) => HttpResponse::InternalServerError().finish(),
      ApiError::NotFound => HttpResponse::NotFound().finish(),
    }
  }
}