// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb_cmd;
mod devices;
mod sad;

use adb_cmd::ADBCommand;
use std::env;

use devices::Device;
use sad::SADError;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            adb_list_devices,
            adb_list_packages,
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
