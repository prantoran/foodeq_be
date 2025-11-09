// region:    --- Modules

mod dev_db; // recreate and seed dev db

use tokio::sync::OnceCell;
use tracing::info;

use crate::model::ModelManager; // once lockish

// endregion: --- Modules

/// Initialize environment for local development.
/// (for early development, will be called from main())
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:12} - init_dev()", "FOR-DEV-ONLY");
        dev_db::init_dev_db().await.unwrap(); // unwrap ok for dev only, can break early but its fine
    })
    .await;
}

/// Initialize test environment
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    // returns a reference to the ModelManager
    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap() // better to crash early in tests than proceed with invalid state
        })
        .await;
    
    mm.clone()
}