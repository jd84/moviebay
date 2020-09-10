use super::{FutRes, Model, Table};
use crate::sqlite::{params, Connection, SharedDb};
use serde::{Deserialize, Serialize};

macro_rules! mk_movie {
    ($x:expr) => {
        Ok(Movie {
            id: $x.get(0)?,
            tmdb_id: $x.get(1)?,
            title: $x.get(2)?,
            overview: $x.get(3)?,
            release_year: $x.get(4)?,
            file_path: $x.get(5)?,
            poster_path: $x.get(6)?,
            backdrop_path: $x.get(7)?,
        })
    };
}

/// Represents the tabe movies in the databases
pub struct MovieTable {
    db: SharedDb,
    fields: [&'static str; 8],
    name: String,
}

impl MovieTable {
    /// Create a new handler to the movies table
    pub fn new(db: SharedDb) -> MovieTable {
        let fields = [
            "id",
            "tmdb_id",
            "title",
            "overview",
            "release_year",
            "file_path",
            "poster_path",
            "backdrop_path",
        ];
        MovieTable {
            db,
            fields,
            name: "movies".to_owned(),
        }
    }
}

impl Table for MovieTable {
    type Model = Movie;

    fn create_table(&self) -> FutRes<()> {
        let db = self.db.clone();

        let func = async move {
            db.spawn(Box::new(|conn: &Connection| {
                conn.execute(
                    "CREATE TABLE movies (
                    id              INTEGER PRIMARY KEY,
                    tmdb_id         INTEGER,
                    title           VARCHAR(255) NOT NULL,
                    overview        TEXT,
                    release_year    INTEGER NOT NULL,
                    file_path       VARCHAR(255) NOT NULL,
                    poster_path     VARCHAR(255),
                    backdrop_path   VARCHAR(255)
                )",
                    params![],
                )
            }))
            .await?;
            Ok(())
        };
        Box::pin(func)
    }

    fn by_id(&self, id: i32) -> FutRes<Option<Self::Model>> {
        let db = self.db.clone();
        let select = format!(
            "SELECT {} FROM {} WHERE ID=?1 LIMIT 1",
            &self.fields.join(","),
            self.get_name()
        );

        let func = async move {
            let movie = db
                .spawn(Box::new(move |conn: &Connection| {
                    let mut stmt = conn.prepare(&select)?;
                    let movie_iter = stmt.query_map(params![id], |row| mk_movie!(row))?;

                    let mut movies = movie_iter.map(|m| m.unwrap()).collect::<Vec<_>>();

                    let movie;
                    if movies.len() != 1 {
                        movie = None;
                    } else {
                        movie = Some(movies.pop().unwrap());
                    }
                    Ok(movie)
                }))
                .await?;
            Ok(movie)
        };
        Box::pin(func)
    }

    fn all(&self) -> FutRes<Vec<Self::Model>> {
        let db = self.db.clone();
        let select = format!("SELECT {} FROM {}", &self.fields.join(","), self.get_name());
        let func = async move {
            let movies = db
                .spawn(Box::new(move |conn: &Connection| {
                    let mut stmt = conn.prepare(&select)?;
                    let movie_iter = stmt.query_map(params![], |row| mk_movie!(row))?;
                    let movies = movie_iter.map(|m| m.unwrap()).collect::<Vec<_>>();
                    Ok(movies)
                }))
                .await?;
            Ok(movies)
        };
        Box::pin(func)
    }

    fn save(&self, model: Self::Model) -> FutRes<()> {
        let db = self.db.clone();
        let insert = format!(
            "INSERT INTO {} ({}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            self.get_name(),
            &self.fields.join(",")
        );
        let func = async move {
            db.spawn(Box::new(move |conn: &Connection| {
                conn.execute(
                    &insert,
                    params![
                        model.tmdb_id,
                        model.title,
                        model.overview,
                        model.release_year,
                        model.file_path,
                        model.poster_path,
                        model.backdrop_path
                    ],
                )
            }))
            .await?;
            Ok(())
        };
        Box::pin(func)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub tmdb_id: i32,
    pub title: String,
    pub overview: String,
    pub release_year: i32,
    pub file_path: String,
    pub poster_path: String,
    pub backdrop_path: String,
}

impl Model for Movie {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DatabaseConfig;
    use crate::sqlite::Runtime;

    #[test]
    fn test_by_id() {
        let func = async {
            let config = DatabaseConfig {
                name: "test.db".to_owned(),
            };
            let (db, rt) = Runtime::channel(config);
            rt.run();
            let t = MovieTable::new(db);
            t.create_table().await.unwrap();

            let movie = t.by_id(1).await.unwrap();
            assert_eq!(None, movie);

            let mut newmovie = Movie {
                id: 0,
                tmdb_id: 0,
                title: "Test Movie".to_owned(),
                overview: "".into(),
                release_year: 2020,
                file_path: "/test_file.mkv".to_owned(),
                poster_path: "/test_poster.jpg".to_owned(),
                backdrop_path: "/test_backdrop.jpg".to_owned(),
            };
            t.save(newmovie.clone()).await.unwrap();
            newmovie.id = 1;

            let movie = t.by_id(1).await.unwrap();
            assert_eq!(Some(newmovie), movie);
        };

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(func);
    }

    #[test]
    fn test_all() {
        let func = async {
            let config = DatabaseConfig {
                name: "test.db".to_owned(),
            };
            let (db, rt) = Runtime::channel(config);
            rt.run();
            let t = MovieTable::new(db);
            t.create_table().await.unwrap();

            let movie = Movie {
                id: 0,
                tmdb_id: 0,
                title: "Test Movie".to_owned(),
                overview: "".into(),
                release_year: 2020,
                file_path: "/test_file.mkv".to_owned(),
                poster_path: "/test_poster.jpg".to_owned(),
                backdrop_path: "/test_backdrop.jpg".to_owned(),
            };
            t.save(movie).await.unwrap();

            let movies = t.all().await.unwrap();
            assert_eq!(1, movies.len());
        };

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(func);
    }
}
