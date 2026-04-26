// rs_sender.rs

// sends focus score data points to frontend graph for app

use tauri::Emitter;

pub fn send_focus_update(app: &tauri::AppHandle, score: f32) {
    app.emit("focus-update", (score * 100.0).round() as i32).unwrap();
}