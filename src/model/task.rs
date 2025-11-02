use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow; // allow us to read data from sqlx and translate into a struct

// region: -- Task Types

// Serialize for serializing to JSON when called by an api
#[derive(Clone, Debug, Serialize, FromRow)] // Clone: need to send copy back to the client
pub struct Task {
    pub id: u64,
    pub title: String,
    // pub description: String,
    // pub completed: bool,
}

// A view onto the tasks table, they are not exhaustive representation of all the fields
// For task creation, we don't want to reset to be able to reset the id of a task, or creator id
// This is sent to the model layer, need to deserialize from JSON
#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
    // pub description: String,
}

// This is sent to the model layer, need to deserialize from JSON
#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
    // pub description: Option<String>,
    // pub completed: Option<bool>,
}
// endregion: -- Task Types

// region: -- TaskBmc

pub struct TaskBmc;

impl TaskBmc {
    // Create a new task, returns the id of the created task
    // &Ctx is the context created by the web layer for the request, but &Ctx is decoupled from frameworks like Axum. Hence, downstream users such as Model layer can use &Ctx without knowing about Axum.
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate,
    ) -> Result<i64> {
        let db = mm.db(); // we are within the model layer, so we can access the db pool
        let (id,) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO tasks (title) VALUES ($1) RETURNING id",
        ) // using parameterized query to prevent SQL injection
        .bind(task_c.title)
        .fetch_one(db)
        .await?;

        Ok(id)
    }
}

// endregion: -- TaskBmc
