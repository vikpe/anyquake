// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use serde_json::{json, Value};

use anyquake_core::qutil;
use anyquake_core::modules::ModuleLike;
use anyquake_core::modules::ezquake::EzQuake;


#[tauri::command]
fn get_quake_info() -> Value {
    let installations: Vec<String> = qutil::find_pak0_paths(&PathBuf::from("~"));
    return json!(installations);
}

#[tauri::command]
fn install_ezquake() -> Value {
    let ez = EzQuake{};
    let res = match ez.uninstall() {
        Ok(_) => "ok",
        Err(_) => "not ok",
    };
    return json!(res);
}



fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![get_quake_info, install_ezquake])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
