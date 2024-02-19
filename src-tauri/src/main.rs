// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
fn maximize(window: tauri::Window) -> Result<(), String> {
    window.maximize().map_err(|e| e.to_string())
}

#[tauri::command]
fn close(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            minimize,
            maximize,
            close,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
