// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb_cmd;
mod devices;
mod packages;
mod sad;
mod users;

use packages::Package;
use serde::{Deserialize, Serialize};
use std::env;

use devices::Device;
use sad::SADError;
use users::User;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            adb_list_devices_with_users,
            adb_list_packages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceWithUsers {
    device: Device,
    users: Vec<User>,
}

#[tauri::command]
fn adb_list_devices_with_users() -> Result<Vec<DeviceWithUsers>, SADError> {
    let mut device_with_users: Vec<DeviceWithUsers> = vec![];

    let acd = devices::ADBTerminalImpl {};
    let acu = users::ADBTerminalImpl {};
    let res = acd.list_devices();
    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(devices) => {
            for device in devices {
                let res = acu.list_users(device.id.to_owned());

                match res {
                    Err(e) => {
                        return Err(SADError::E(e));
                    }
                    Ok(users) => device_with_users.push(DeviceWithUsers { device, users }),
                }
            }

            return Ok(device_with_users);
        }
    }
}

#[tauri::command]
fn adb_list_packages(device_id: &str, user_id: &str) -> Result<Vec<Package>, SADError> {
    let acl = packages::ADBTerminalImpl {};
    let res = acl.list_packages(device_id.to_string(), user_id.to_string());
    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(o) => {
            return Ok(o);
        }
    }
}
