mod config;
mod db;
mod web;

use actix_web::{web::Data, App, HttpServer};

use sqlx::postgres::PgPoolOptions;

pub struct AppData {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub music_path: String,
}

#[actix_web::main]
async fn main() {
    let config: config::Config = toml::from_str(&std::fs::read_to_string("./zelda.toml").unwrap()).unwrap();
    let pool = {
        let config::Database { port, host, user, password } = config.database;
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&format!("postgres://{user}:{password}@{host}:{port}"))
            .await.unwrap()
    };

    for e in std::fs::read_dir(&config.music_path).unwrap().into_iter().filter_map(|e| e.ok()) {
        db::scan_album(&pool, e.path()).await.unwrap();
    }

    let config::Web { port, ref host, .. } = config.web;
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default().allowed_origin("http://192.168.1.91:5173");
        App::new()
            .app_data(Data::new(AppData {
                pool: pool.clone(),
                music_path: config.music_path.clone(),
            }))
            .service(web::index::albums)
            .service(web::index::tracks)
            .service(web::index::cover)
            .service(web::index::stream)
            .service(actix_files::Files::new("/", config.web.ui_path.clone()))
            .wrap(cors)
    })
        .bind((host.to_string(), port)).unwrap()
        .run().await.unwrap();
}
