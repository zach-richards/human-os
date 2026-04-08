// window_info.rs

use active_win_pos_rs::get_active_window;

use crate::sys::windows::window_context;

pub struct WindowInfo {
    pub id: String,
    pub app_name: String,
    pub title: String,
    pub context: String,
    pub timestamps: Vec<std::time::Instant>,
}

impl WindowInfo {
    pub fn new(id: String, app_name: &str, title: &str) -> Self {
        Self {
            id,
            app_name: app_name.to_string(),
            title: title.to_string(),
            context: Self::assess_window_context(app_name, title),
            timestamps: Vec::new(),
        }
    }

    pub fn update_timestamp(&mut self) {
        self.timestamps.push(std::time::Instant::now());
    }

    pub fn time_spent(&self) -> std::time::Duration {
        if self.timestamps.len() < 2 {
            return std::time::Duration::new(0, 0);
        }
        let mut total = std::time::Duration::new(0, 0);
        for i in (1..self.timestamps.len()).step_by(2) {
            total += self.timestamps[i] - self.timestamps[i - 1];
        }
        total
    }

    pub fn assess_window_context(app_name: &str, title: &str) -> String {
        window_context::classify_window_context(app_name, title).to_string()
    }
}

// Minimal ActiveWindow placeholder used by insert_window_info.
// Replace with platform-specific implementation that queries the OS.
struct ActiveWindow {
    pub app_name: String,
    pub title: String,
    pub process_id: u32,
    pub window_id: u32,
}

fn insert_window_info() -> Option<WindowInfo> {
    if let Ok(win) = get_active_window() {
        let info = WindowInfo::new(win.window_id, &win.app_name, &win.title);
        Some(info)
    } else {
        None
    }
}
