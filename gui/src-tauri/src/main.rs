// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use serde_json::{json, Value};

use anyquake_core::modules::ezquake::EzQuake;
use anyquake_core::modules::ModuleLike;
use anyquake_core::qutil;

#[tauri::command]
fn get_quake_info() -> Value {
    let installations: Vec<String> = qutil::find_pak0_paths(&PathBuf::from("~"));
    return json!(installations);
}

#[tauri::command]
fn uninstall_ezquake() -> Value {
    let ez = EzQuake {};
    let res: String = match ez.uninstall() {
        Ok(_) => String::from("ok"),
        Err(err) => String::from(err.to_string()),
    };
    return json!(res);
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![get_quake_info, uninstall_ezquake])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
