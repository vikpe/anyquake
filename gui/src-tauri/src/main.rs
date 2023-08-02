// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use anyhow::{anyhow, Result as AnyhowResult};
use serde_json::Value;

use anyquake_core::commands::Command;
use anyquake_core::modules::ModuleCollection;
use anyquake_core::qutil;

#[tauri::command]
fn get_quake_info() -> Value {
    let installations: Vec<String> = qutil::find_pak0_paths(&PathBuf::from("~"));
    installations.into()
}

#[tauri::command]
async fn anyquake_command(command: Command) -> String {
    match process_anyquake_command(command).await {
        Ok(output) => output,
        Err(error) => format!("{}", error),
    }
}

async fn process_anyquake_command(command: Command) -> AnyhowResult<String> {
    let modules: ModuleCollection = ModuleCollection {};
    match command {
        Command::Install { module_id: id } => {
            modules.by_id(&id)?.install().await
        }
        Command::Uninstall { module_id: id } => {
            modules.by_id(&id)?.uninstall()
        }
        _ => {
            Err(anyhow!("Invalid command."))
        }
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![anyquake_command, get_quake_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
