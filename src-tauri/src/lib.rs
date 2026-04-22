// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod ui;
mod main;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    tauri::Builder::default()
        .setup(|app| {
            // 1. start tray
            ui::tray_icon::setup_tray(app.handle());

            // 2. start background engine
            std::thread::spawn(|| {
                main::run_engine();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            ui::tray_icon::update_focus_fuel
        ])
        .run(tauri::generate_context!())
        .expect("error running app");
}