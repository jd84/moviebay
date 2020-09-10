mod types;
use crate::config::TmdbConfig;
use bytes::buf::BufExt as _;
use hyper::Client;
use hyper_tls::HttpsConnector;
use types::{Configuration, MovieSearch};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub async fn fetch_configuration(config: TmdbConfig) -> Result<Configuration> {
    let url = format!(
        "https://api.themoviedb.org/3/configuration?api_key={}",
        &config.api_key
    );

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url.parse().unwrap()).await?;
    let body = hyper::body::aggregate(res).await?;

    let configuration: Configuration = serde_json::from_reader(body.reader())?;

    Ok(configuration)
}

pub async fn search_movie(config: TmdbConfig, name: &str, year: i32) -> Result<MovieSearch> {
    let url = format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&language=en&query={}&year={}",
        &config.api_key, name, year
    );

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url.parse().unwrap()).await?;
    let body = hyper::body::aggregate(res).await?;

    let search: MovieSearch = serde_json::from_reader(body.reader())?;

    Ok(search)
}
