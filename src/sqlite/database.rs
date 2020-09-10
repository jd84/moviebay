use super::SharedDb;
use crate::config::DatabaseConfig;
use rusqlite::backup;
use rusqlite::{Connection, Result as SqlResult};
use std::sync::{mpsc as std_mpsc, Arc, Mutex};
use std::time;
use tokio::sync::mpsc as tokio_mpsc;

type ResultSender<T> = tokio_mpsc::Sender<SqlResult<T>>;

trait Runable: Send + 'static {
    fn run(&self, conn: &Connection);
}

struct Job<F, R> {
    func: Box<F>,
    sender: ResultSender<R>,
}

impl<F, R> Job<F, R>
where
    F: Fn(&Connection) -> SqlResult<R>,
    F: Send + 'static,
    R: Send + 'static,
{
    fn run_now(&self, conn: &Connection) {
        let res = (self.func)(conn);
        let mut tx = self.sender.clone();
        tokio::spawn(async move { tx.send(res).await });
    }
}

impl<F, R> Runable for Job<F, R>
where
    F: Fn(&Connection) -> SqlResult<R>,
    F: Send + 'static,
    R: Send + 'static,
{
    fn run(&self, conn: &Connection) {
        self.run_now(conn);
    }
}

/// An async `rusqlite` handler. This handler communicates
/// with the `Runtime` ans spawn sql jobs on it.
pub struct AsyncSqlite {
    inner: Arc<Mutex<std_mpsc::Sender<Box<dyn Runable>>>>,
    config: DatabaseConfig,
}

impl AsyncSqlite {
    /// Put `AsyncSqlite` into an `Arc` to share the connection
    /// between threads safe.
    pub fn into_shared(self) -> SharedDb {
        Arc::new(self)
    }
    /// Spawn a sql job
    pub async fn spawn<F, R>(&self, f: Box<F>) -> Result<R, Box<dyn std::error::Error>>
    where
        F: Fn(&Connection) -> SqlResult<R>,
        F: Send + 'static,
        R: Send + 'static,
    {
        let (res_tx, mut res_rx) = tokio_mpsc::channel::<SqlResult<R>>(100);

        let job = Box::new(Job {
            func: f,
            sender: res_tx,
        });

        let tx = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            tx.lock().unwrap().send(job).expect("failed to send job");
        })
        .await?;

        match res_rx.recv().await {
            Some(r) => Ok(r?),
            None => unimplemented!(),
        }
    }

    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.config.name.clone();
        self.spawn(Box::new(move |conn: &Connection| {
            let mut dst = Connection::open(&name)?;
            let backup = backup::Backup::new(conn, &mut dst)?;
            backup.run_to_completion(
                5,
                time::Duration::from_millis(250),
                Some(|p| {
                    println!("{:?}", p);
                }),
            )
        }))
        .await?;
        Ok(())
    }
}

/// A `rusqlite` runtime wihch can be safly used in an async context.
/// The runtime takes care of the execution of the statements.
///
/// # Example
///
/// ```
/// let (sqlite, rt) = Runtime::channel();
/// rt.run(); // Spawn runtime on tokio
///
/// // execute sql
/// sqlite.spawn(Box::new(|conn: &Connection| Ok(0))).await;
/// ```
pub struct Runtime {
    conn: Connection,
    rx: std_mpsc::Receiver<Box<dyn Runable>>,
}

impl Runtime {
    /// Returns an `AsyncSqlite` handler to spawn sql jobs and
    /// the `Runtime` itself.
    pub fn channel(config: DatabaseConfig) -> (SharedDb, Runtime) {
        let (tx, rx) = std_mpsc::channel();
        let conn = Connection::open_in_memory().unwrap();
        (
            AsyncSqlite {
                inner: Arc::new(Mutex::new(tx)),
                config,
            }
            .into_shared(),
            Runtime { conn, rx },
        )
    }

    /// Spawn self on tokio executor
    pub fn run(self) {
        tokio::task::spawn_blocking(move || {
            while let Ok(job) = self.rx.recv() {
                job.run(&self.conn);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DatabaseConfig;
    use rusqlite::params;

    #[test]
    fn test_async_sql() {
        let func = async {
            let config = DatabaseConfig {
                name: "test.db".to_owned(),
            };
            let (sqlite, rt) = Runtime::channel(config);
            rt.run();

            let res = sqlite
                .spawn(Box::new(|conn: &Connection| {
                    conn.execute(
                        "CREATE TABLE person (
                            id      INTEGER PRIMARY KEY,
                            name    TEXT NOT NULL,
                            data    BLOB
                        )",
                        params![],
                    )
                }))
                .await
                .unwrap();

            assert_eq!(0, res);
        };

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(func);
    }
}
