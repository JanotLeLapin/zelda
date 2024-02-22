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
        let cors = actix_cors::Cors::default().allowed_origin("http://192.168.1.91:5173");
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(web::index::albums)
            .service(web::index::cover)
            .service(web::index::stream)
            .wrap(cors)
    })
        .bind(("0.0.0.0", 4000)).unwrap()
        .run().await.unwrap();
}
