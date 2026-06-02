#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

// ── Default images dir (AppData/images) ──
fn default_images_dir(app: &AppHandle) -> PathBuf {
    let dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to resolve app data dir")
        .join("images");
    fs::create_dir_all(&dir).ok();
    dir
}

// ── Resolve the active images dir ──
// If custom_dir is provided and non-empty, use it; otherwise use AppData/images.
fn resolve_images_dir(app: &AppHandle, custom_dir: &Option<String>) -> PathBuf {
    if let Some(dir) = custom_dir {
        if !dir.is_empty() {
            let p = PathBuf::from(dir);
            fs::create_dir_all(&p).ok();
            return p;
        }
    }
    default_images_dir(app)
}

// ── DB lives in AppData always (never moves) ──
fn db_path(app: &AppHandle) -> PathBuf {
    let dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to resolve app data dir");
    fs::create_dir_all(&dir).ok();
    dir.join("db.json")
}

// ─────────────────────────────────────────
//  DATABASE
// ─────────────────────────────────────────

#[tauri::command]
fn load_db(app: AppHandle) -> String {
    match fs::read_to_string(db_path(&app)) {
        Ok(c) => c,
        Err(_) => r#"{"models":[],"images":[],"settings":{}}"#.to_string(),
    }
}

#[tauri::command]
fn save_db(app: AppHandle, data: String) -> Result<(), String> {
    fs::write(db_path(&app), data).map_err(|e| e.to_string())
}

// ─────────────────────────────────────────
//  FOLDER PICKER
// ─────────────────────────────────────────

/// Open a native folder-picker dialog and return the chosen path.
/// Returns an empty string if the user cancelled.
#[tauri::command]
async fn pick_images_folder(app: AppHandle) -> Result<String, String> {
    let result = tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_title("Choose folder to store images")
        .pick_folder();

    match result {
        Some(path) => {
            // Make sure the folder exists
            fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            Ok(path.to_string_lossy().to_string())
        }
        None => Ok(String::new()), // user cancelled
    }
}

/// Return the default AppData images path (shown as fallback in UI).
#[tauri::command]
fn get_default_images_dir(app: AppHandle) -> String {
    default_images_dir(&app).to_string_lossy().to_string()
}

// ─────────────────────────────────────────
//  IMAGE FILE OPERATIONS
//  All accept an optional custom_dir.
//  Pass null/empty to use AppData default.
// ─────────────────────────────────────────

#[tauri::command]
fn save_image_file(
    app: AppHandle,
    filename: String,
    data: Vec<u8>,
    custom_dir: Option<String>,
) -> Result<String, String> {
    let dir = resolve_images_dir(&app, &custom_dir);
    let safe_name = Path::new(&filename)
        .file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();
    let path = dir.join(&safe_name);
    fs::write(&path, &data).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn load_image_file(
    app: AppHandle,
    filename: String,
    custom_dir: Option<String>,
) -> Result<Vec<u8>, String> {
    let dir = resolve_images_dir(&app, &custom_dir);
    let safe_name = Path::new(&filename)
        .file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();
    let path = dir.join(&safe_name);
    // If not found in custom dir, fall back to default AppData dir
    if !path.exists() {
        let fallback = default_images_dir(&app).join(&safe_name);
        if fallback.exists() {
            return fs::read(&fallback).map_err(|e| e.to_string());
        }
    }
    fs::read(&path).map_err(|e| format!("Cannot read {}: {}", safe_name, e))
}

#[tauri::command]
fn delete_image_file(
    app: AppHandle,
    filename: String,
    custom_dir: Option<String>,
) -> Result<(), String> {
    let dir = resolve_images_dir(&app, &custom_dir);
    let safe_name = Path::new(&filename)
        .file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();
    let path = dir.join(&safe_name);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_images_dir(app: AppHandle, custom_dir: Option<String>) -> String {
    resolve_images_dir(&app, &custom_dir)
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
fn list_image_files(app: AppHandle, custom_dir: Option<String>) -> Vec<String> {
    let dir = resolve_images_dir(&app, &custom_dir);
    match fs::read_dir(&dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect(),
        Err(_) => vec![],
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_db,
            save_db,
            pick_images_folder,
            get_default_images_dir,
            save_image_file,
            load_image_file,
            delete_image_file,
            get_images_dir,
            list_image_files,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running ArtVault");
}
