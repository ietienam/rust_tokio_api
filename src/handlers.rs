use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use crate::models::{CreateBook, UpdateBook};
use crate::error::ApiError;

pub async fn create_book(
    pool: web::Data<PgPool>, 
    book: web::Json<CreateBook>
) -> Result<HttpResponse, ApiError> {
    let result = sqlx::query_as!(
        Book,
        "INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id, title, author",
        book.title,
        book.author
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn get_book(
    pool: web::Data<PgPool>, 
    book_id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    match sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", book_id.into_inner())
        .fetch_optional(pool.get_ref())
        .await? {
            Some(book) => Ok(HttpResponse::Ok().json(book)),
            None => Ok(HttpResponse::NotFound().json(json!({
                "error": "Book not found"
            })))
        }
}

pub async fn update_book(
    pool: web::Data<PgPool>, 
    book_id: web::Path<i32>, 
    book: web::Json<UpdateBook>
) -> Result<HttpResponse, ApiError> {
    let result = sqlx::query_as!(
        Book,
        "UPDATE books SET 
            title = COALESCE($1, title), 
            author = COALESCE($2, author) 
         WHERE id = $3
         RETURNING id, title, author",
        book.title,
        book.author,
        book_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match result {
        Some(updated_book) => Ok(HttpResponse::Ok().json(updated_book)),
        None => Ok(HttpResponse::NotFound().json(json!({
            "error": "Book not found"
        })))
    }
}

pub async fn delete_book(
    pool: web::Data<PgPool>, 
    book_id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    match sqlx::query!(
        "DELETE FROM books WHERE id = $1 RETURNING id", 
        book_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await? {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().json(json!({
            "error": "Book not found"
        })))
    }
}