// window_info.rs

// An instance of a window with the info needed to process switching and context

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

    pub fn update_title(&mut self, new_title: &str) {
        self.title = new_title.to_string();
        self.context = Self::assess_window_context(&self.app_name, new_title);
    }

    pub fn update_context(&mut self) {
        self.context = Self::assess_window_context(&self.app_name, &self.title);
    }

    pub fn update_timestamp(&mut self) {
        self.timestamps.push(std::time::Instant::now());
    }

    /*pub fn time_spent(&self) -> std::time::Duration {
        if self.timestamps.len() < 2 {
            return std::time::Duration::new(0, 0);
        }
        let mut total = std::time::Duration::new(0, 0);
        for i in (1..self.timestamps.len()).step_by(2) {
            total += self.timestamps[i] - self.timestamps[i - 1];
        }
        total
    }*/

    pub fn assess_window_context(app_name: &str, title: &str) -> String {
        window_context::classify_window_context(app_name, title).to_string()
    }
}