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

    for e in std::fs::read_dir("/var/music").unwrap().into_iter().filter_map(|e| e.ok()) {
        db::scan_album(&pool, e.path()).await.unwrap();
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(web::index::page)
            .service(web::index::cover)
    })
        .bind(("0.0.0.0", 4000)).unwrap()
        .run().await.unwrap();
}
