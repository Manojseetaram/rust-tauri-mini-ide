#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, process::Command};
use tauri::command;

#[command]
fn run_rust_code(code: String) -> Result<String, String> {
    // Save code to file
    fs::write("temp.rs", code).map_err(|e| e.to_string())?;

    // Compile
    let compile_status = Command::new("rustc")
        .arg("temp.rs")
        .arg("-o")
        .arg("temp_exec")
        .status()
        .map_err(|e| e.to_string())?;

    if !compile_status.success() {
        return Err("Compilation failed".into());
    }

    // Run
    let output = Command::new("./temp_exec")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_rust_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
