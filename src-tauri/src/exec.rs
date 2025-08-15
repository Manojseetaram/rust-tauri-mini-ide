use std::{fs, process::Command};
use tauri::command;

#[command]
pub fn run_rust_code(code: String) -> Result<String, String> {
    fs::write("temp.rs", &code).map_err(|e| e.to_string())?;

    let compile_status = Command::new("rustc")
        .arg("temp.rs")
        .arg("-o")
        .arg("temp_exec")
        .status()
        .map_err(|e| e.to_string())?;

    if !compile_status.success() {
        return Err("Compilation failed".into());
    }

    let output = Command::new("./temp_exec")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
