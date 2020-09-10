use std::fmt;

#[derive(Debug)]
pub enum Error {
    Database(Box<dyn std::error::Error>),
    Sqlite(rusqlite::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom Error")
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Error {
        Error::Database(err)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::Sqlite(err)
    }
}

impl std::error::Error for Error {}
