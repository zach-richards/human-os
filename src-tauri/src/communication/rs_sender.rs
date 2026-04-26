// rs_sender.rs

use tauri::Emitter;

pub fn send_focus_update(app: &tauri::AppHandle, score: f32) {
    app.emit("focus-update", (score * 100.0).round() as i32).unwrap();
}