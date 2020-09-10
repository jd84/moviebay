use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::Arc;

pub type SharedCfg = Arc<Config>;

/// Codec settings used with FFmpeg
#[derive(Debug, Clone, Deserialize)]
pub struct CodecConfig {
    pub args: Vec<String>,
}

/// Settings for Database
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub name: String,
}

/// Settings for FFmpeg
#[derive(Debug, Clone, Deserialize)]
pub struct FFmpegConfig {
    pub bin: String,
    pub codecs: HashMap<String, CodecConfig>,
}

/// Settings for the library
#[derive(Debug, Clone, Deserialize)]
pub struct LibraryConfig {
    pub movies: String,
}

/// Settings for the The Movie Database
#[derive(Debug, Clone, Deserialize)]
pub struct TmdbConfig {
    pub api_key: String,
}

/// The base `Config` for moviebay
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ffmpeg: FFmpegConfig,
    pub library: LibraryConfig,
    pub database: DatabaseConfig,
    pub tmdb: TmdbConfig,
}

impl Config {
    pub fn new() -> SharedCfg {
        Config::default().into_shared()
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> io::Result<Config> {
        let mut contents = String::new();
        let mut file = File::open(file)?;
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    pub fn into_shared(self) -> SharedCfg {
        Arc::new(self)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::from_file("moviebay.toml").unwrap()
    }
}
