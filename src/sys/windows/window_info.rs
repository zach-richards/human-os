// window_info.rs

pub struct WindowInfo {
    pub id: u32,
    pub title: &str,
    pub context: Option<&str>,
    pub timestamps: Vec<std::time::Instant>,
}

impl WindowInfo {
    pub fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title,
            context: Some(Self::assess_context(&title)),
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

    pub fn assess_context(&mut self, title: &str) -> &str {

        let context = classify_window(title);

        context
    }
}

fn insert_window_info() -> WindowInfo {
    if let Ok(win) = get_active_window() {
        WindowInfo::new(win.window_id, &win.title)
        println!("App: {}", win.app_name);
        println!("Title: {}", win.title);
        println!("PID: {}", win.process_id);
        println!("Window ID: {}", win.window_id);
    }
}
