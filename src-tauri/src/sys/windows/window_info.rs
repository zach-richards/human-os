use std::time::Instant;
use crate::sys::windows::window_context;

#[derive(Clone, Debug)]
pub struct WindowInfo {
    pub id: String,
    pub app_name: String,
    pub title: String,
    pub context: String,

    // keep only meaningful timestamps (not unlimited growth)
    pub last_seen: Option<Instant>,
    pub focus_count: u32,
}

impl WindowInfo {
    pub fn new(id: String, app_name: &str, title: &str) -> Self {
        let clean_id = if id.is_empty() {
            "unknown".to_string()
        } else {
            id
        };

        let clean_app = if app_name.is_empty() {
            "unknown".to_string()
        } else {
            app_name.to_string()
        };

        let clean_title = if title.is_empty() {
            "unknown".to_string()
        } else {
            title.to_string()
        };

        Self {
            id: clean_id.clone(),
            app_name: clean_app,
            title: clean_title.clone(),
            context: Self::assess_window_context(&clean_id, &clean_title),
            last_seen: Some(Instant::now()),
            focus_count: 1,
        }
    }

    pub fn update_title(&mut self, new_title: &str) {
        let title = if new_title.is_empty() {
            "unknown"
        } else {
            new_title
        };

        self.title = title.to_string();
        self.context = Self::assess_window_context(&self.app_name, title);
        self.touch();
    }

    pub fn update_context(&mut self) {
        self.context = Self::assess_window_context(&self.app_name, &self.title);
        self.touch();
    }

    pub fn update_timestamp(&mut self) {
        self.focus_count = self.focus_count.saturating_add(1);
        self.touch();
    }

    fn touch(&mut self) {
        self.last_seen = Some(Instant::now());
    }

    pub fn assess_window_context(app_name: &str, title: &str) -> String {
        window_context::classify_window_context(app_name, title).to_string()
    }
}