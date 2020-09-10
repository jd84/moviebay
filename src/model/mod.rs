mod error;
mod movie;

pub use movie::{Movie, MovieTable};

use error::Error;
use std::future::Future;
use std::pin::Pin;

pub type Result<T, E = Error> = std::result::Result<T, E>;
pub(crate) type FutRes<T> = Pin<Box<dyn Future<Output = Result<T>> + Send + Sync>>;

// #[async_trait]
pub trait Model {
    // fn fetch() -> Self;
    // async fn by_id(t: impl Table, id: i32) -> Self;
}

pub trait Table {
    type Model;

    /// Create the table
    fn create_table(&self) -> FutRes<()>;

    /// Trys to fetch a model by id
    fn by_id(&self, id: i32) -> FutRes<Option<Self::Model>>;

    /// Get all entries from the table
    /// WARNING: This can take a lot of resouces depending on the
    /// table size
    fn all(&self) -> FutRes<Vec<Self::Model>>;

    /// Saves a model
    fn save(&self, model: Self::Model) -> FutRes<()>;

    /// Get the name of the table
    fn get_name(&self) -> &str;
}
