use anyhow::Result;
use log::info;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{str::FromStr, fs};

// https://github.com/tauri-apps/tauri/discussions/5557
// https://github.com/RandomEngy/tauri-sqlite/blob/main/src-tauri/src/main.rs
pub async fn init(app_handle: &tauri::AppHandle) -> Result<SqlitePool> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");

    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");

    let sqlite_path = app_dir.join("sad.sqlite");

    info!("db_url {:?}", sqlite_path);

    let db_url = sqlite_path
        .to_str()
        .expect("unable to get db_path is from PathBuf");

    let mut conn_options = SqliteConnectOptions::from_str(db_url)?;
    conn_options = conn_options.create_if_missing(true);

    let db = SqlitePool::connect_with(conn_options).await?;
    sqlx::migrate!().run(&db).await?;

    return Ok(db);
}
