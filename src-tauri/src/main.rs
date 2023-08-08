// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb_cmd;
mod devices;
mod sad;
mod users;

use adb_cmd::ADBCommand;
use serde::{Deserialize, Serialize};
use std::env;

use devices::Device;
use sad::SADError;
use users::User;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            adb_list_devices,
            adb_list_packages,
            adb_list_users,
            adb_list_devices_with_users,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn adb_list_devices() -> Result<Vec<Device>, SADError> {
    let ac = devices::ADBTerminalImpl {};
    let res = ac.list_devices();
    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(o) => {
            return Ok(o);
        }
    }
}

#[tauri::command]
fn adb_list_users() -> Result<Vec<User>, SADError> {
    let ac = users::ADBTerminalImpl {};
    let device_id = String::from("115f26ee");
    let res = ac.list_users(device_id);

    match res {
        Err(e) => {
            return Err(SADError::E(e));
        }
        Ok(o) => {
            return Ok(o);
        }
    }
}

#[tauri::command]
fn adb_list_packages() -> Result<String, String> {
    let res = adb_cmd::ADBShell::new(&["pm", "list", "packages"]).execute();
    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(o) => {
            let ot = o.replace("package:", "");
            let ots = ot.trim();
            // for l in ots.lines() {
            // }
            return Ok(format!("{}", ots));
        }
    }
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
                    Ok(users) => device_with_users.push(DeviceWithUsers {
                        device,
                        users,
                    }),
                }
            }

            return Ok(device_with_users);
        }
    }
}
