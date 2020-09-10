use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    images: Images,
}

#[derive(Debug, Deserialize)]
pub struct Images {
    base_url: String,
    secure_base_url: String,
    backdrop_sizes: Vec<String>,
    logo_sizes: Vec<String>,
    poster_sizes: Vec<String>,
    profile_sizes: Vec<String>,
    still_sizes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MovieResult {
    poster_path: String,
    adult: bool,
    overview: String,
    release_date: String,
    genre_ids: Vec<i32>,
    id: i32,
    original_title: String,
    original_language: String,
    title: String,
    backdrop_path: Option<String>,
    popularity: f32,
    vote_count: i32,
    video: bool,
    vote_average: f32,
}

#[derive(Debug, Deserialize)]
pub struct MovieSearch {
    page: i32,
    total_results: i32,
    total_pages: i32,
    results: Vec<MovieResult>,
}
