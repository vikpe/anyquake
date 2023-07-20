// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{json, Value};

use anyquake_core::add;

use crate::qutil::QuakeInstallation;

mod qutil;


#[tauri::command]
fn get_quake_info() -> Value {
    println!("elo: {}", add(1, 2));
    let installations: Vec<QuakeInstallation> = qutil::get_installations();
    return json!(installations);
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![get_quake_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
