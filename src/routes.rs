use actix_web::web;
use crate::handlers::{create_book, get_book, update_book, delete_book};

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/books")
    .route("", web::post().to(create_book))
    .route("/{id}", web::get().to(get_book))
    .route("/{id}", web::put().to(update_book))
    .route("/{id}", web::delete().to(delete_book))
  );
}