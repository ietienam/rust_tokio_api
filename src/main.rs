mod config;
mod db;
mod models;
mod handlers;
mod routes;
mod error;

use actix_web::{App, HttpServer, web};
use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
