use anyhow::Ok;
use sqlx::SqlitePool;

use crate::config;
use crate::store;
use anyhow::{anyhow, Result};

pub struct Cache {
    pub devices_store: store::Store,
    config: Option<config::Config>,
}

impl Cache {
    pub fn new() -> Self {
        return Self {
            devices_store: store::Store::new(),
            config: None,
        };
    }

    pub async fn get_config(&mut self, db: &SqlitePool) -> Result<config::Config> {
        if self.config.is_none() {
            let config_svc = config::SqliteImpl { db };
            let dcnfg = config_svc.get_default_config().await?;
            self.set_config(dcnfg);
        }

        match &self.config {
            Some(c) => Ok(c.clone()),
            None => Err(anyhow!("cache: config not found")),
        }
    }

    pub fn set_config(&mut self, config: config::Config) {
        self.config = Some(config)
    }
}
