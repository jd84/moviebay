use crate::config::{Config, LibraryConfig};
use regex::Regex;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VideoFile {
    pub title: String,
    pub release_year: i32,
    pub path: PathBuf,
    pub poster_path: String,
    pub backdrop_path: String,
}

pub struct Scanner {
    config: Arc<LibraryConfig>,
    movies: Vec<VideoFile>,
    patterns: Vec<Regex>,
}

impl Scanner {
    pub fn new(config: Arc<Config>) -> Scanner {
        let config = Arc::new(config.library.clone());
        let regex = Regex::new(r"^([a-zA-Z0-9_ ]*)\((\d+)\)").unwrap();
        Scanner {
            config,
            movies: Vec::new(),
            patterns: vec![regex],
        }
    }

    fn visit_dir(
        &mut self,
        dir: &Path,
        cb: &dyn Fn(&DirEntry, &mut Vec<VideoFile>),
    ) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit_dir(&path, cb)?;
                } else {
                    cb(&entry, &mut self.movies);
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<&[VideoFile]> {
        let movie_path = self.config.movies.clone();
        let re = self.patterns[0].clone();

        self.visit_dir(
            &Path::new(&movie_path),
            &|entry: &DirEntry, movies: &mut Vec<VideoFile>| {
                let file_name = entry.file_name().into_string().unwrap();
                if re.is_match(&file_name) {
                    let caps = re.captures(&file_name).unwrap();
                    let title = &caps[1].trim();
                    let release_date = &caps[2];

                    let movie = VideoFile {
                        title: title.to_string(),
                        release_year: release_date.parse().unwrap(),
                        path: entry.path(),
                        poster_path: "/c6WuCykIy8GAsitJOTk1DEga1ML.jpg".to_owned(),
                        backdrop_path: "/qDVdTL1KqGmGLGsy7UPA3vksku7.jpg".to_owned(),
                    };
                    movies.push(movie);
                }
            },
        )?;

        Ok(&self.movies)
    }
}
