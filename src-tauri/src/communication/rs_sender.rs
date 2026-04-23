// rs_sender.rs

use tauri::Manager;

use crate::auxillary::*;

fn send_score(app: &tauri::AppHandle) {
    auxillary::get_color_from_score(score);
    app.emit("my-event", "Hello from Rust!").unwrap();
}