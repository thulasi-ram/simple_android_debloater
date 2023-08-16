// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb_cmd;
mod devices;
mod packages;
mod sad;
mod store;
mod users;

use packages::Package;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use store::DeviceWithUserPackages;
use tauri::Manager;
use tokio::sync::mpsc;

use devices::Device;
use sad::SADError;
use users::User;

struct AsyncEventSender {
    inner: tokio::sync::Mutex<mpsc::Sender<String>>,
}

struct SadCache {
    inner: tokio::sync::Mutex<store::Store>,
}

fn main() {
    let (async_event_sender, mut async_event_receiver) = mpsc::channel(1);
    let store: store::Store = HashMap::new();

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

fn event_publisher<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    manager
        .emit_all("rs2js", format!("rs: {}", message))
        .unwrap();
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
                        cache.insert(
                            du.device.id.to_owned(),
                            DeviceWithUserPackages::new_from_device_with_users(du.clone()),
                        );
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
    let res = acl.list_packages(device_id.to_string(), user_id.to_string());
    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(o) => {
            let mut cache = cache_state.inner.lock().await;
            let device = cache.get_mut(&device_id.to_string()).expect("device is invalid");
            let user = device.user(user_id.to_string()).expect("user is invalid");

            for p in o.to_vec() {
                user.add_package(p);
            }

            return Ok(o);
        }
    }
}

#[tauri::command]
async fn adb_disable_clear_and_stop_packages(
    device_id: &str,
    user_id: &str,
    pkg: &str,
    event_sender_state: tauri::State<'_, AsyncEventSender>,
    cache_state: tauri::State<'_, SadCache>,
) -> Result<(), SADError> {

    let cache = cache_state.inner.lock().await;
    println!("{:?}", cache);
    let esender: tokio::sync::MutexGuard<'_, mpsc::Sender<String>> =
        event_sender_state.inner.lock().await;
    let _res = esender
        .send(String::from("hey"))
        .await
        .map_err(|e| e.to_string());

    let acl = packages::ADBTerminalImpl {};
    let res = acl.disable_package(device_id.to_string(), user_id.to_string(), pkg.to_string());
    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(o) => {
            return Ok(o);
        }
    }
}
