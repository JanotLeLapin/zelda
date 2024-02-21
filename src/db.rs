use sqlx::{Pool, Postgres, Error, FromRow};
use std::{fs, path::PathBuf};

#[derive(FromRow, Debug)]
pub struct Album {
    pub path: String,
    pub name: String,
    pub cover_mime: String,
    pub cover: Vec<u8>,
}

#[derive(FromRow, Debug)]
pub struct Track {
    pub path: String,
    pub album: String,
    pub name: String,
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn scan_album(pool: &Pool<Postgres>, path: PathBuf) -> Result<()> {
    let mut tracks = vec![];
    let mut name = None;
    let mut cover = None;
    for e in fs::read_dir(&path).unwrap().filter_map(|e| e.ok()) {
        let p = e.path();
        let ext = p.extension().and_then(|ext| ext.to_str());
        match ext {
            Some("flac") | Some("mp3") | Some("m4a") => {
                let tags = audiotags::Tag::new().read_from_path(&p).unwrap();

                match tags.title() {
                    Some(title) => tracks.push(Track {
                        path: p.to_str().unwrap().to_string(),
                        name: title.to_string(),
                        album: p.parent().unwrap().to_str().unwrap().to_string(),
                    }),
                    None => {},
                };
                if name.is_none() { match tags.album_title() {
                    Some(title) => name = Some(title.to_string()),
                    None => {},
                } }
                if cover.is_none() { match tags.album_cover() {
                    Some(pic) => {
                        cover = Some((match pic.mime_type {
                            audiotags::MimeType::Tiff => "tiff",
                            audiotags::MimeType::Jpeg => "jpg",
                            audiotags::MimeType::Gif => "gif",
                            audiotags::MimeType::Bmp => "bmp",
                            audiotags::MimeType::Png => "png",
                        }.to_string(), pic.data.to_vec()));
                    },
                    None => {},
                } }
            },
            Some("png") | Some("bmp") | Some("gif") | Some("jpg") | Some("tiff") => match std::fs::read(&p) {
                Ok(data) => cover = Some((ext.unwrap().to_string(), data)),
                Err(_) => {},
            }
            _ => {},
        };
    };

    let (mime, data) = cover.unwrap();

    sqlx::query("INSERT INTO albums (path, name, cover_mime, cover) VALUES ($1, $2, $3, $4) ON CONFLICT (path) DO NOTHING")
        .bind(path.to_str().unwrap())
        .bind(name.unwrap())
        .bind(mime)
        .bind(data)
        .execute(pool).await?;

    for track in tracks {
        sqlx::query("INSERT INTO tracks (path, album, name) VALUES ($1, $2, $3) ON CONFLICT (path) DO NOTHING")
            .bind(track.path)
            .bind(path.to_str().unwrap())
            .bind(track.name)
            .execute(pool).await?;
    }

    Ok(())
}
