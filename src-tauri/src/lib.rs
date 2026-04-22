mod sys;
mod logic;
mod notifications;
mod ui;
mod engine;
mod state;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // tray
            ui::tray_icon::setup_tray(app.handle());

            // background engine
            std::thread::spawn(|| {
                engine::run_engine();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            ui::tray_icon::update_focus_fuel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}