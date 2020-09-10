mod api;
mod config;
mod context;
mod ffmpeg;
mod model;
mod scan;
mod sqlite;
mod tmdb;

use crate::api::MakeApiSvc;
use crate::config::Config;
use crate::context::Context;
use crate::model::{MovieTable, Table};
use crate::scan::Scanner;
use crate::sqlite::{params, Connection};
use hyper::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::from_config(Config::new());
    let sqlite = ctx.db();
    let config = ctx.cfg();

    let tmdb_cfg = tmdb::fetch_configuration(config.tmdb.clone())
        .await
        .unwrap();
    let movies = tmdb::search_movie(config.tmdb.clone(), "Collateral", 2004)
        .await
        .unwrap();

    // println!("{:?}", tmdb_cfg);
    // println!("{:?}", movies);
    
    let table = MovieTable::new(sqlite.clone());
    table.create_table().await?;

    let mut scanner = Scanner::new(config.clone());
    let movies = scanner.run()?;

    for movie in movies {
        println!("{:?}", movie);

        let m = movie.clone();
        sqlite
            .spawn(Box::new(move |conn: &Connection| {
                conn.execute(
                    "INSERT INTO movies (tmdb_id, title, overview, release_year, file_path, poster_path,  backdrop_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![0, m.title, "", m.release_year, m.path.to_str(), m.poster_path, m.backdrop_path],
                )
            }))
            .await?;
    }

    sqlite.save().await?;

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(MakeApiSvc::new(config, sqlite));
    println!("Listening on http://{}", addr);

    server.await?;
    Ok(())
}
