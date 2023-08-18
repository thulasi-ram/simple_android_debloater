// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb_cmd;
mod devices;
mod err;
mod events;
mod packages;
mod sad;
mod store;
mod users;

use anyhow::anyhow;
use err::ResultOkPrintErrExt;
use events::PackageEvent;
use packages::Package;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::sync::mpsc;

use devices::Device;
use sad::SADError;
use users::User;

struct AsyncEventSender {
    inner: tokio::sync::Mutex<mpsc::Sender<events::AsyncEvent>>,
}

struct SadCache {
    inner: tokio::sync::Mutex<store::Store>,
}

fn main() {
    let (async_event_sender, mut async_event_receiver): (
        mpsc::Sender<events::AsyncEvent>,
        mpsc::Receiver<events::AsyncEvent>,
    ) = mpsc::channel(1);
    let store = store::Store::new();

    tauri::Builder::default()
        .manage(AsyncEventSender {
            inner: tokio::sync::Mutex::new(async_event_sender),
        })
        .manage(SadCache {
            inner: tokio::sync::Mutex::new(store),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            adb_list_devices_with_users,
            adb_list_packages,
            adb_disable_clear_and_stop_packages,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_event_receiver.recv().await {
                        event_publisher(output, &app_handle);
                    }
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
    cache_state: tauri::State<'_, SadCache>,
) -> Result<Vec<DeviceWithUsers>, SADError> {
    let mut device_with_users: Vec<DeviceWithUsers> = vec![];

    let acd = devices::ADBTerminalImpl {};
    let acu = users::ADBTerminalImpl {};
    let res = acd.list_devices();
    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(devices) => {
            let mut cache = cache_state.inner.lock().await;

            for device in devices {
                let res = acu.list_users(device.id.to_owned());

                match res {
                    Err(e) => {
                        return Err(SADError::E(e));
                    }
                    Ok(users) => {
                        let du = DeviceWithUsers { device, users };
                        cache.insert_device_with_user(du.clone());
                        device_with_users.push(du);
                    }
                }
            }

            return Ok(device_with_users);
        }
    }
}

#[tauri::command]
async fn adb_list_packages(
    device_id: &str,
    user_id: &str,
    cache_state: tauri::State<'_, SadCache>,
) -> Result<Vec<Package>, SADError> {
    let acl = packages::ADBTerminalImpl {};
    let packages = acl.list_packages(device_id.to_string(), user_id.to_string())?;

    let mut cache = cache_state.inner.lock().await;
    let device = cache.device(device_id.to_owned())?;
    let user = device.user(user_id.to_owned())?;
    for p in packages.clone() {
        user.add_package(p.clone())
    }
    return Ok(packages);
}

#[tauri::command]
async fn adb_disable_clear_and_stop_packages(
    device_id: &str,
    user_id: &str,
    pkg: &str,
    event_sender_state: tauri::State<'_, AsyncEventSender>,
    cache_state: tauri::State<'_, SadCache>,
) -> Result<(), SADError> {
    let acl = packages::ADBTerminalImpl {};
    acl.disable_package(device_id.to_string(), user_id.to_string(), pkg.to_string())?;

    {
        let mut cache = cache_state.inner.lock().await;
        let device = cache.device(device_id.to_owned())?;
        let user = device.user(user_id.to_owned())?;
        let package = user.get_package(pkg);

        match package {
            None => {
                return Err(anyhow!("package {} not found in cache", pkg.to_string()).into());
            }
            Some(p) => {
                p.set_state(packages::PackageState::Disabled);
                let pe = PackageEvent::new(device_id.to_string(), user_id.to_string(), p.clone());
                let esender = event_sender_state.inner.lock().await;
                esender
                    .send(Box::new(pe))
                    .await
                    .ok_or_print_err("error emitting");
            }
        }
    }

    return Ok(());
}
