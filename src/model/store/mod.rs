// region:     --- Modules

pub mod error;

pub use self::error::{Error, Result};

use crate::config; // db url from config
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion:  --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5) 
        .connect(&config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string())) // Convert err to string in order to serialize
        // sqlx implements this as macro
}

