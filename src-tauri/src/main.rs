// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adb_cmd::ADBCommand;
use std::env;

mod adb_cmd;

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
fn adb_list_devices() -> Result<String, String> {
    let res = adb::ADBRaw::new(&["devices"]).execute();
    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(o) => {
            let ot = o.replace("List of devices attached", "");
            let ots = ot.trim();
            // for l in ots.lines() {
            // }
            return Ok(format!("{}", ots));
        }
    }
}

#[tauri::command]
fn adb_list_packages() -> Result<String, String> {
    // let res = adb::ADBShell::new(&["pm list packages"]).execute();
    let res = adb::ADBShell::new(&["pm", "list", "packages"]).execute();
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