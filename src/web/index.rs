use crate::db::{Result, Album, Track};

use actix_web::{get, http::header, web, HttpResponse};
use sqlx::{Pool, Postgres};

markup::define! {
    Index(albums: Vec<Album>) {
        @markup::doctype()
        html {
            body {
                .albums {
                    @for album in albums {
                        .album {
                            h3 { @album.name }
                        }
                    }
                }
            }
        }
    }
}

#[get("/")]
pub async fn page(data: web::Data<Pool<Postgres>>) -> HttpResponse {
    let albums = sqlx::query_as::<sqlx::Postgres, Album>("SELECT * FROM albums")
        .fetch_all(data.get_ref()).await.unwrap();

    HttpResponse::Ok()
        .insert_header(header::ContentType::html())
        .body(Index { albums }.to_string())
}
