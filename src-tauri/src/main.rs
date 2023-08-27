// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb_cmd;
mod cache;
mod config;
mod db;
mod devices;
mod err;
mod events;
mod packages;
mod sad;
mod store;
mod users;

use std::{env, time::Duration};

use anyhow::anyhow;
use config::Config;
use err::{ResultOkPrintErrExt, IntoSADError};
use events::{Event, PackageEvent};
use log::error;
use packages::Package;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use futures;

use tauri::Manager;
use tokio::{sync::{mpsc}, time};


use devices::Device;
use sad::SADError;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};
use users::User;

struct App {
    db: tokio::sync::Mutex<anyhow::Result<SqlitePool>>,
    pub event_emitter: tokio::sync::Mutex<mpsc::Sender<events::AsyncEvent>>,
    pub cache: tokio::sync::Mutex<cache::Cache>,
}

impl App {
    fn new(s: mpsc::Sender<events::AsyncEvent>) -> Self {
        Self {
            db: tokio::sync::Mutex::new(Err(anyhow!("connection is none"))),
            event_emitter: tokio::sync::Mutex::new(s),
            cache: tokio::sync::Mutex::new(cache::Cache::new()),
        }
    }

    async fn config(&self) -> anyhow::Result<Config> {
        let db_res = self.db.lock().await;
        let db = db_res.as_ref().unwrap();
        let cache = &mut self.cache.lock().await;
        let config = cache.get_config(db).await?;
        return Ok(config);
    }
}

#[tokio::main]
async fn main() {
    fix_path_env::fix().unwrap();

    let (async_event_sender, mut async_event_receiver): (
        mpsc::Sender<events::AsyncEvent>,
        mpsc::Receiver<events::AsyncEvent>,
    ) = mpsc::channel(1);

    let app = App::new(async_event_sender);

    #[cfg(debug_assertions)]
    const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];

    #[cfg(not(debug_assertions))]
    const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(app)
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .with_colors(ColoredLevelConfig::default())
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            adb_list_devices_with_users,
            adb_list_packages,
            adb_disable_clear_and_stop_package,
            adb_enable_package,
            adb_install_package,
            get_config,
            update_config,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            
            let init_db_fut = async move {
                let conn = db::init(&app_handle).await.expect("unable to init db");
                let app_state: tauri::State<App> = app_handle.state();
                let mut app_db = app_state.db.lock().await;
                *app_db = Ok(conn);
            };

            futures::executor::block_on(init_db_fut);

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_event_receiver.recv().await {
                        event_publisher(output, &app_handle);
                    }
                }
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let mut interval = time::interval(Duration::from_millis(3000));
                loop {
                    interval.tick().await;
                    track_devices(&app_handle).await;
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Since we pass events via mpsc channel a serializable trait is not object safe
/// So we resort to deserialization before passing event and then again serialize in emit_all of tauri
fn event_publisher<R: tauri::Runtime>(event: events::AsyncEvent, manager: &impl Manager<R>) {
    let pl: serde_json::Value = serde_json::from_str(&event.epayload().unwrap()).unwrap();
    manager.emit_all(&event.etype().to_string(), pl).unwrap();
}

async fn track_devices<R: tauri::Runtime>(manager: &impl Manager<R>) {
    let app: tauri::State<'_, App> = manager.state();
    let config = app.config().await.unwrap();

    let res = _adb_list_device_with_users(config).await;
    match res {
        Err(e) => {
            error!("Error getting async devices {:?}", e);
        }
        Ok(device_with_users) => {
            let w = manager.get_window("main").unwrap();
            let mut cache = app.cache.lock().await;

            for du in device_with_users {
                let event = events::DeviceEvent::new(du.clone());
                let pl: serde_json::Value =
                    serde_json::from_str(&event.epayload().unwrap()).unwrap();
                cache.devices_store.insert_device_with_user(du);

                let res = w.emit_all(&event.etype().to_string(), pl);
                match res {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error emitting async devices {:?}", e);
                    }
                }
            }
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceWithUsers {
    device: Device,
    users: Vec<User>,
}

#[tauri::command]
async fn adb_list_devices_with_users(
    app: tauri::State<'_, App>,
) -> Result<Vec<DeviceWithUsers>, SADError> {
    let config = app.config().await.into_sad_error("unable to get config")?;

    let res = _adb_list_device_with_users(config).await;
    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(device_with_users) => {
            let mut cache = app.cache.lock().await;

            for du in device_with_users.clone() {
                cache.devices_store.insert_device_with_user(du.clone());
            }

            return Ok(device_with_users);
        }
    }
}

async fn _adb_list_device_with_users(
    config: config::Config,
) -> anyhow::Result<Vec<DeviceWithUsers>> {
    let mut device_with_users: Vec<DeviceWithUsers> = vec![];

    let acd = devices::ADBTerminalImpl {
        adb_path: config.custom_adb_path.to_owned(),
    };
    let acu = users::ADBTerminalImpl {
        adb_path: config.custom_adb_path.to_owned(),
    };
    let devices = acd.list_devices()?;

    for device in devices {
        let users: Vec<User> = acu.list_users(device.id.to_owned())?;
        device_with_users.push(DeviceWithUsers { device, users });
    }

    return anyhow::Ok(device_with_users);
}

#[tauri::command]
async fn adb_list_packages(
    device_id: &str,
    user_id: &str,
    app: tauri::State<'_, App>,
) -> Result<Vec<Package>, SADError> {
    let config = app.config().await.unwrap();

    let acl = packages::ADBTerminalImpl::new(config.custom_adb_path);
    let packages = acl.list_packages(device_id.to_string(), user_id.to_string())?;

    let mut cache = app.cache.lock().await;
    let device = cache.devices_store.device(device_id.to_owned())?;
    let user = device.user(user_id.to_owned())?;
    for p in packages.clone() {
        user.add_package(p.clone())
    }
    return Ok(packages);
}

#[tauri::command]
async fn adb_disable_clear_and_stop_package(
    device_id: &str,
    user_id: &str,
    pkg: &str,
    app: tauri::State<'_, App>,
) -> Result<(), SADError> {
    let config = app.config().await.into_sad_error("unable to get config")?;

    let acl = packages::ADBTerminalImpl::new(config.custom_adb_path);
    acl.disable_package(
        device_id.to_string(),
        user_id.to_string(),
        pkg.to_string(),
        config.clear_packages_on_disable,
    )?;

    {
        let mut cache = app.cache.lock().await;
        let device = cache.devices_store.device(device_id.to_owned())?;
        let user = device.user(user_id.to_owned())?;
        let package = user.get_package(pkg);

        match package {
            None => {
                return Err(anyhow!("package {} not found in cache", pkg.to_string()).into());
            }
            Some(p) => {
                p.set_state(packages::PackageState::Disabled);
                let pe = PackageEvent::new(device_id.to_string(), user_id.to_string(), p.clone());
                let esender = app.event_emitter.lock().await;
                esender
                    .send(Box::new(pe))
                    .await
                    .ok_or_print_err("error emitting");
            }
        }
    }

    return Ok(());
}

#[tauri::command]
async fn adb_enable_package(
    device_id: &str,
    user_id: &str,
    pkg: &str,
    app: tauri::State<'_, App>,
) -> Result<(), SADError> {
    let config = app.config().await.into_sad_error("unable to get config")?;

    let acl = packages::ADBTerminalImpl::new(config.custom_adb_path);
    acl.enable_package(device_id.to_string(), user_id.to_string(), pkg.to_string())?;

    {
        let mut cache = app.cache.lock().await;
        let device = cache.devices_store.device(device_id.to_owned())?;
        let user = device.user(user_id.to_owned())?;
        let package = user.get_package(pkg);

        match package {
            None => {
                return Err(anyhow!("package {} not found in cache", pkg.to_string()).into());
            }
            Some(p) => {
                p.set_state(packages::PackageState::Enabled);
                let pe = PackageEvent::new(device_id.to_string(), user_id.to_string(), p.clone());
                let esender = app.event_emitter.lock().await;
                esender
                    .send(Box::new(pe))
                    .await
                    .ok_or_print_err("error emitting");
            }
        }
    }

    return Ok(());
}

#[tauri::command]
async fn adb_install_package(
    device_id: &str,
    user_id: &str,
    pkg: &str,
    app: tauri::State<'_, App>,
) -> Result<(), SADError> {
    let config = app.config().await.into_sad_error("unable to get config")?;

    let acl = packages::ADBTerminalImpl::new(config.custom_adb_path);
    acl.install_package(device_id.to_string(), user_id.to_string(), pkg.to_string())?;

    {
        let mut cache = app.cache.lock().await;
        let device = cache.devices_store.device(device_id.to_owned())?;
        let user = device.user(user_id.to_owned())?;
        let package = user.get_package(pkg);

        match package {
            None => {
                return Err(
                    anyhow!("package {} not found in cache for install", pkg.to_string()).into(),
                );
            }
            Some(p) => {
                p.set_state(packages::PackageState::Enabled);
                let pe = PackageEvent::new(device_id.to_string(), user_id.to_string(), p.clone());
                let esender = app.event_emitter.lock().await;
                esender
                    .send(Box::new(pe))
                    .await
                    .ok_or_print_err("error emitting");
            }
        }
    }

    return Ok(());
}

#[tauri::command]
async fn get_config(app: tauri::State<'_, App>) -> Result<config::Config, SADError> {
    let db_guard = &app.db.lock().await;
    let db_conn = db_guard.as_ref().into_sad_error("")?;
    let mut cache = app.cache.lock().await;
    let svc = config::SqliteImpl { db: db_conn };
    let res = svc.get_default_config().await;
    match res {
        Ok(r) => {
            cache.set_config(r.clone());
            return Ok(r);
        }
        Err(e) => {
            return Err(SADError::E(e.into()));
        }
    }
}

#[tauri::command]
async fn update_config(config: config::Config, app: tauri::State<'_, App>) -> Result<(), SADError> {
    let db_guard = &app.db.lock().await;
    let db_conn = db_guard.as_ref().into_sad_error("")?;

    let mut cache = app.cache.lock().await;
    let svc = config::SqliteImpl { db: db_conn };
    let res = svc.update_default_config(config).await;

    match res {
        Ok(r) => {
            cache.set_config(r.clone());
            return Ok(());
        }
        Err(e) => {
            return Err(SADError::E(e.into()));
        }
    }
}
