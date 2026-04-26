// lib.rs

mod sys;
mod logic;
mod notifications;
mod ui;
mod engine;
mod auxillary;
mod communication;

use tauri::AppHandle;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Handles the running between the React frontend, Tauri tray icon, and Rust backend in one
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle: &AppHandle = app.handle();
            // tray
            ui::tray_icon::setup_tray(app.handle()).unwrap();

            let handle = handle.clone();

            std::thread::spawn(move || {
                engine::run_engine(&handle);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}