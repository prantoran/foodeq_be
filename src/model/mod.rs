//! Model Layer
//!  
//! Design:
//! 
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g. db_pool, S3 client, redis client, etc.)
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller)
//! - In frameworks like Axum, Tauri, etc. the `ModelManager` are typically used as App State (i.e. 1 model manager, multiple controllers).
//! - ModelManager are designed to be passed as an argument
//!   to all Model Controllers functions.

// region:   --- Modules

mod error;
mod store;

pub use self::error::{Error, Result};

pub mod model;

// endregion:  --- Modules