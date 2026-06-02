// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

// ── Get the app data directory for storing DB JSON ──
fn app_data_dir(app: &AppHandle) -> PathBuf {
    app.path_resolver()
        .app_data_dir()
        .expect("Failed to resolve app data dir")
}

// ── Get the images directory (inside app data) ──
fn images_dir(app: &AppHandle) -> PathBuf {
    let dir = app_data_dir(app).join("images");
    fs::create_dir_all(&dir).ok();
    dir
}

// ─────────────────────────────────────────
//  DATABASE — read/write the JSON metadata
// ─────────────────────────────────────────

/// Load the full DB JSON string from disk. Returns empty object string if not found.
#[tauri::command]
fn load_db(app: AppHandle) -> String {
    let path = app_data_dir(&app).join("db.json");
    match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(_) => r#"{"models":[],"images":[]}"#.to_string(),
    }
}

/// Save the full DB JSON string to disk.
#[tauri::command]
fn save_db(app: AppHandle, data: String) -> Result<(), String> {
    let dir = app_data_dir(&app);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("db.json");
    fs::write(&path, data).map_err(|e| e.to_string())
}

// ─────────────────────────────────────────
//  IMAGES — save / load / delete files
// ─────────────────────────────────────────

/// Save an image file to the images directory.
/// `filename` is just the base filename (e.g. "myimage_1234.jpg").
/// `data` is the raw bytes as a Vec<u8> (sent from JS as base64-decoded array).
#[tauri::command]
fn save_image_file(app: AppHandle, filename: String, data: Vec<u8>) -> Result<String, String> {
    let dir = images_dir(&app);
    // Sanitize filename — strip any path separators
    let safe_name = Path::new(&filename)
        .file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();
    let path = dir.join(&safe_name);
    fs::write(&path, &data).map_err(|e| e.to_string())?;
    // Return the absolute path so the frontend can use it as a src URI
    Ok(path.to_string_lossy().to_string())
}

/// Load an image file from the images directory and return its bytes.
/// The frontend converts bytes → blob URL for display.
#[tauri::command]
fn load_image_file(app: AppHandle, filename: String) -> Result<Vec<u8>, String> {
    let dir = images_dir(&app);
    let safe_name = Path::new(&filename)
        .file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();
    let path = dir.join(&safe_name);
    fs::read(&path).map_err(|e| format!("Cannot read {}: {}", safe_name, e))
}

/// Delete an image file from the images directory.
#[tauri::command]
fn delete_image_file(app: AppHandle, filename: String) -> Result<(), String> {
    let dir = images_dir(&app);
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

/// Return the absolute path of the images directory (for display purposes).
#[tauri::command]
fn get_images_dir(app: AppHandle) -> String {
    images_dir(&app).to_string_lossy().to_string()
}

/// List all filenames in the images directory (for re-syncing).
#[tauri::command]
fn list_image_files(app: AppHandle) -> Vec<String> {
    let dir = images_dir(&app);
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
            save_image_file,
            load_image_file,
            delete_image_file,
            get_images_dir,
            list_image_files,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running ArtVault");
}
