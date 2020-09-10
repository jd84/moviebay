use crate::config::Config;
use crate::ffmpeg::FFmpeg;
use crate::model::{MovieTable, Table};
use crate::sqlite::SharedDb;
use hyper::{header, Body, Response, StatusCode};
use std::sync::Arc;

macro_rules! json {
    ($x:expr) => {
        match serde_json::to_string($x) {
            Ok(json) => Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("INTERNAL_SERVER_ERROR".into())
                .unwrap(),
        }
    };
}

pub async fn get_movies(db: SharedDb) -> Result<Response<Body>, hyper::Error> {
    let table = MovieTable::new(db);
    let movies = table.all().await.unwrap();
    Ok(json!(&movies))
}

pub async fn get_movie(db: SharedDb, id: i32) -> Result<Response<Body>, hyper::Error> {
    let table = MovieTable::new(db);
    let movie = table.by_id(id).await.unwrap();
    Ok(json!(&movie))
}

pub async fn get_stream(
    db: SharedDb,
    config: Arc<Config>,
    id: i32,
) -> Result<Response<Body>, hyper::Error> {
    let table = MovieTable::new(db);
    // let _movie = table.by_id(id).await.unwrap();

    let config = Arc::new(config.ffmpeg.clone());
    let ffmpeg = FFmpeg::new(config);
    let (tx, body) = Body::channel();

    tokio::spawn(async move {
        ffmpeg.transcode("Marked for Death (1990).mkv", tx).await;
    });

    let resp = Response::builder()
        .header("Content-Type", "video/mp4")
        .header("Content-Disposition", "inline")
        .header("Content-Transfer-Enconding", "binary")
        .body(body)
        .unwrap();

    Ok(resp)
}
