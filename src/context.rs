use crate::config::SharedCfg;
use crate::sqlite::{Runtime, SharedDb};
use std::sync::Arc;

pub type SharedCtx = Arc<Context>;

pub struct Context {
    db: SharedDb,
    config: SharedCfg,
}

impl Context {
    pub fn from_config(config: SharedCfg) -> SharedCtx {
        let (db, rt) = Runtime::channel(config.database.clone());
        rt.run();

        Arc::new(Context { db, config })
    }

    pub fn db(&self) -> SharedDb {
        self.db.clone()
    }

    pub fn cfg(&self) -> SharedCfg {
        self.config.clone()
    }
}
