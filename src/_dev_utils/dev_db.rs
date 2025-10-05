use std::{fs, path::PathBuf};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;


type Db = Pool<Postgres>;

const PG_DEV_POSTGRESS_URL: &str = "postgres://postgres:welcome@localhost/postgres"; // default postgres db
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db"; // dev db

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial/";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:12} - init_dev_db()", "FOR-DEV-ONLY");

    // -- Creat the app_db/app_user with the postgres user.
    {   // root_db scope inside braces
        let root_db = new_db_pool(PG_DEV_POSTGRESS_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    // -- Get sql files in the directory
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort(); // we have files sorted by design, so that we can execute in order

    // -- SQL Execute each file
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/"); // windows fix

            // Only take the .sql and skip the SQL_RECREATE_DB file
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }
    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:12} - pexec(): {file}", "FOR-DEV-ONLY");

    // -- Read the sql file
    let content = fs::read_to_string(file)?;

    // TODO: Make the split more sql proof
    let sqls: Vec<&str> = content
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?; // one query at a time
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(500))
        .connect(db_con_url)
        .await
}