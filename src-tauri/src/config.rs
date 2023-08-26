use anyhow::{anyhow, Result};
use core::result::Result::Ok;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Config {
    id: i64,
    prompt_disable_package: bool,
    custom_adb_path: String,
}

static DEFAULT_CONFIG_ID: i64 = 1;

impl Default for Config {
    fn default() -> Self {
        Config {
            id: DEFAULT_CONFIG_ID,
            prompt_disable_package: true,
            custom_adb_path: String::from(""),
        }
    }
}

// async traits needs another supporting crate for now
// https://www.reddit.com/r/rust/comments/gt1ct3/using_sqlx_with_nonasync_code/
// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
// pub trait GetDefaultConfig {
//     fn get_default_config(&self) -> Result<Config>;
// }

// pub trait UpdateDefaultConfig {
//     fn update_default_config(&self, new_config: Config) -> Result<Config>;
// }

pub struct SqliteImpl<'a> {
    pub db: &'a SqlitePool,
}

impl SqliteImpl<'_> {
    pub async fn get_default_config(&self) -> Result<Config> {
        let res = sqlx::query_as!(
            Config,
            "SELECT * FROM config where id = ?",
            DEFAULT_CONFIG_ID
        )
        // .bind(DEFAULT_CONFIG_ID)
        .fetch_one(self.db)
        .await;

        match res {
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    return Ok(Config::default());
                }
                _ => {
                    return Err(anyhow!("error executing db: {}", e.to_string()).into());
                }
            },
            Ok(r) => {
                return Ok(r);
            }
        }
    }

    pub async fn update_default_config(&self, config: Config) -> Result<()> {
        let res = sqlx::query(
            "insert into config(id, prompt_disable_package, custom_adb_path) VALUES($1, $2, $3) 
            ON CONFLICT(id) DO UPDATE SET prompt_disable_package = $2, custom_adb_path = $3",
        )
        .bind(DEFAULT_CONFIG_ID)
        .bind(config.prompt_disable_package)
        .bind(config.custom_adb_path)
        .execute(self.db)
        .await?;

        if res.rows_affected() <= 0 {
            return Err(anyhow!("no rows updated").into());
        }

        return Ok(());
    }
}
