#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod exec;
mod fs_ops;
mod hander;


use tauri::Builder;

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            fs_ops::read_file,
            fs_ops::write_file,
            fs_ops::create_file,
            fs_ops::create_folder,
            fs_ops::delete_path,
            hander::list_folder,
            hander::list_files,
            exec::run_rust_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
