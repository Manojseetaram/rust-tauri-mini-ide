// use std::{fs, path::Path};
// use tauri::command;

// #[command]
// pub fn list_folder(path: String) -> Result<Vec<String>, String> {
//     let path = Path::new(&path);
//     if !path.exists() {
//         return Err("Directory does not exist".into());
//     }

//     let mut entries = vec![];
//     for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
//         let entry = entry.map_err(|e| e.to_string())?;
//         let mut name = entry.file_name().into_string().unwrap_or_default();
//         if entry.path().is_dir() {
//             name.push('/'); // mark folders
//         }
//         entries.push(name);
//     }

//     Ok(entries)
// }

// #[command]
// pub fn list_files(dir: String) -> Result<Vec<String>, String> {
//     let path = Path::new(&dir);
//     if !path.exists() {
//         return Err("Directory does not exist".into());
//     }
//     let mut files = vec![];
//     for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
//         let entry = entry.map_err(|e| e.to_string())?;
//         files.push(entry.file_name().into_string().unwrap_or_default());
//     }
//     Ok(files)
// }
