// rs_sender.rs

use tauri::Emitter;
use crate::logic::cognitive_model::FocusState;

pub fn send_state_to_frontend(app: &tauri::AppHandle, state: FocusState) {
    let color = match state {
        FocusState::Fatigued => (248, 113, 113),
        FocusState::Distracted => (236, 175, 117),
        FocusState::Neutral => (217, 231, 122),
        FocusState::Focus => (52, 211, 153),
        FocusState::Flow => (96, 165, 250),
    };

    app.emit("graph-color", color).unwrap();
}