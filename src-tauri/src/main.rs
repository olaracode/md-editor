// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;
use dirs::home_dir;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref APP_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
}

#[derive(Serialize)]
struct NotionFile {
    filename: String,
    content: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {} from tauri", name)
}

#[tauri::command]
fn get_files() -> Result<Vec<NotionFile>, String> {
    let app_dir = APP_DIR.lock().unwrap();
    let mut files = Vec::new();
    if let Some(dir) = &*app_dir {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                if entry.path().is_file() {
                    let filename = entry
                        .path()
                        .file_name()
                        .and_then(|name| name.to_str())
                        .ok_or("Failed to read filename")?
                        .to_string();
                    let content = fs::read_to_string(&entry.path()).map_err(|e| e.to_string())?;
                    let file = NotionFile { filename, content };
                    files.push(file);
                }
            }
        }
    }
    Ok(files)
}

#[tauri::command]
fn create_file(filename: String, content: String) -> Result<(), String> {
    let app_dir = APP_DIR.lock().unwrap();
    if let Some(dir) = &*app_dir {
        let mut file_path = dir.clone();
        file_path.push(filename);
        let mut file = fs::File::create(file_path).map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn delete_file(filename: String) -> Result<String, String> {
    let app_dir = APP_DIR.lock().unwrap();
    if let Some(dir) = &*app_dir {
        let mut file_path = dir.clone();
        file_path.push(filename);
        fs::remove_file(file_path).map_err(|e| e.to_string())?;
    }
    Ok(String::from("File deleted successfully"))
}

fn main() {
    tauri::Builder::default()
        .setup(|_| {
            let mut path = home_dir().expect("Failed to get home directory");
            path.push(".notionless");
            if let Err(e) = fs::create_dir_all(&path) {
                eprintln!("Failed to create directory: {}", e);
            } else {
                let mut app_dir = APP_DIR.lock().unwrap();
                *app_dir = Some(path);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_files,
            create_file,
            delete_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
