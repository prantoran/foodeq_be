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
    // Should return the id of the created task
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate,
    ) -> Result<i64> {
        let db = mm.db(); // we are within the model layer, so we can access the db pool
        let (id,) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO task (title) VALUES ($1) RETURNING id",
        ) // using parameterized query to prevent SQL injection
        .bind(task_c.title)
        .fetch_one(db)
        .await?;

        Ok(id)
    }
}

// endregion: -- TaskBmc

// region: --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[serial] // ensure tests run serially to avoid db conflicts
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title"; // prefix fixtures with fx

        // -- Execute
        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };
        let created_id = TaskBmc::create(&ctx, &mm, task_c).await?;

        // -- Verify
        let (title,): (String,) = sqlx::query_as::<_, (String,)>(
            "SELECT title FROM task WHERE id = $1",
        )
        .bind(created_id)
        .fetch_one(mm.db())
        .await?;

        assert_eq!(title, fx_title.to_string());

        // -- Cleanup
        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(created_id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(count, 1, "Did not delete 1 row in cleanup?");

        Ok(())
    }
}

// endregion: --- Tests