mod db;
mod web;

use actix_web::{web::Data, App, HttpServer};

use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432")
        .await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(web::index::page)
    })
        .bind(("0.0.0.0", 4000)).unwrap()
        .run().await.unwrap();
}
