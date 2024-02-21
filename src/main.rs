mod db;

use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432")
        .await.unwrap();

    let root = "/var/music/JESUS IS KING";
    db::scan_album(&pool, root.into()).await.unwrap();
}
