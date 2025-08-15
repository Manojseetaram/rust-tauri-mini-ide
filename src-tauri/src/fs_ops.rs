use std::{fs, path::PathBuf};
use tauri::command;

#[command]
pub fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(PathBuf::from(path))
        .map_err(|e| format!("Error reading file: {}", e))
}

#[command]
pub fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(PathBuf::from(path), content)
        .map_err(|e| format!("Error writing file: {}", e))
}

#[command]
pub fn create_file(path: String) -> Result<(), String> {
    fs::write(PathBuf::from(path), "")
        .map_err(|e| format!("Error creating file: {}", e))
}

#[command]
pub fn create_folder(path: String) -> Result<(), String> {
    fs::create_dir_all(PathBuf::from(path))
        .map_err(|e| format!("Error creating folder: {}", e))
}

#[command]
pub fn delete_path(path: String) -> Result<(), String> {
    let p = PathBuf::from(path);
    if p.is_dir() {
        fs::remove_dir_all(p).map_err(|e| format!("Error deleting folder: {}", e))
    } else {
        fs::remove_file(p).map_err(|e| format!("Error deleting file: {}", e))
    }
}
