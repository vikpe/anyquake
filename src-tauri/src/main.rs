// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use rust_search::SearchBuilder;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn download(url: &str) -> String {
    format!("Lets download {}!", url)
}

#[tauri::command]
fn get_quake_info() -> Value {
    let needle: String = "pak0.pak".to_string();
    let search: Vec<String> = SearchBuilder::default()
        .location("/")
        .search_input(needle)
        .limit(1)
        .build()
        .collect();

    return if search.is_empty() {
        json!({
            "pak0_path": "".to_string(),
            "quake_root_dir_path": "".to_string(),
        })
    } else {
        let pak0_path = search.get(0);
        let mut path = PathBuf::from(pak0_path.unwrap().to_string());
        path.pop();
        let my_path: String = format!("{}", path.display());

        json!({
            "pak0_path": pak0_path,
            "quake_root_dir_path": my_path,
        })
    };
}


#[tauri::command]
fn find_file(needle: &str) -> Vec<String> {
    let search: Vec<String> = SearchBuilder::default()
        .location("/")
        .search_input(needle)
        .limit(1)
        .build()
        .collect();

    return search;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, download, find_file, get_quake_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
