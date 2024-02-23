use crate::db::{Album, Track};

use actix_web::{web, get, http::header, Error, HttpResponse};

#[get("/albums")]
pub async fn albums(data: web::Data<crate::AppData>) -> HttpResponse {
    let albums = sqlx::query_as::<sqlx::Postgres, Album>("SELECT * FROM albums")
        .fetch_all(&data.get_ref().pool).await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&albums).unwrap())
}

#[get("/albums/{id}")]
pub async fn tracks(data: web::Data<crate::AppData>, path: web::Path<String>) -> HttpResponse {
    let tracks = sqlx::query_as::<sqlx::Postgres, Track>("SELECT * FROM tracks WHERE album = $1 ORDER BY pos ASC")
        .bind(path.into_inner())
        .fetch_all(&data.get_ref().pool).await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&tracks).unwrap())
}

#[get("/stream/{id}")]
pub async fn stream(data: web::Data<crate::AppData>, path: web::Path<String>) -> HttpResponse {
    let mut file_path = std::path::PathBuf::from(&data.get_ref().music_path);
    file_path.push(path.into_inner().as_str());

    let file = match std::fs::read(&file_path) {
        Ok(data) => data,
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    let body = futures::stream::once(futures::future::ok::<_, Error>(web::Bytes::from(file)));
    HttpResponse::Ok()
        .content_type(match file_path.extension().unwrap().to_str().unwrap() {
            "flac" => "audio/flac",
            "mp3" => "audio/mp3",
            "m4a" => "audio/mp4",
            _ => unimplemented!(),
        })
        .streaming(body)
}

#[get("/cover/{id}")]
pub async fn cover(data: web::Data<crate::AppData>, path: web::Path<String>) -> HttpResponse {
    let (mime, data) = match sqlx::query_as::<sqlx::Postgres, (String, Vec<u8>)>("SELECT cover_mime, cover FROM albums WHERE path = $1")
        .bind(path.into_inner())
        .fetch_one(&data.get_ref().pool).await {
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
