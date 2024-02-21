use crate::db::{Result, Album, Track};

use actix_web::{web, get, http::header, HttpResponse};
use sqlx::{Pool, Postgres};

markup::define! {
    Index(albums: Vec<Album>) {
        @markup::doctype()
        html {
            body {
                .albums {
                    @for album in albums {
                        .album {
                            img[src=format!("/cover/{}", urlencoding::encode(&album.path).to_string()), style="width: 64px;"];
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

#[get("/cover/{id}")]
pub async fn cover(data: web::Data<Pool<Postgres>>, path: web::Path<String>) -> HttpResponse {
    let (mime, data) = match sqlx::query_as::<sqlx::Postgres, (String, Vec<u8>)>("SELECT cover_mime, cover FROM albums WHERE path = $1")
        .bind(path.into_inner())
        .fetch_one(data.get_ref()).await {
            Ok(album) => album,
            Err(_) => return HttpResponse::NotFound().finish(),
        };

    HttpResponse::Ok()
        .insert_header(match mime.as_str() {
            "png" => header::ContentType::png(),
            "jpg" => header::ContentType::jpeg(),
            _ => { unimplemented!() }
        })
        .body(data)
}
