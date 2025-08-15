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
       .plugin(tauri_plugin_dialog::init())
      .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            fs_ops::read_file,
            fs_ops::write_file,
            fs_ops::create_file,
            fs_ops::create_folder,
            fs_ops::delete_path,
             fs_ops::list_folder,
          
            exec::run_rust_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}