#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::PathBuf};

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(PathBuf::from(path))
        .map_err(|err| format!("Error reading file: {}", err))
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(PathBuf::from(path), content)
        .map_err(|err| format!("Error writing file: {}", err))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, write_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
