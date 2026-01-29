// sys_stats.rs

use std::time::Instant;
use std::sync::{Mutex, Arc};

struct SystemStats {
    // Timing
    last_activity: Option<Instant>,
    idle_since: Option<Instant>,

    // Keyboard
    last_key_time: Option<Instant>,
    keyboard_burst_start: Option<Instant>,
    keys_in_burst: u32,

    // Mouse
    mouse_moved: bool,
    mouse_clicked: bool,

    // Scroll wheel
    scroll_active: bool,

    // Window context
    focused_window: Option<String>,
    last_window_change: Option<Instant>,
}

pub type SharedSysStats = Arc<Mutex<SystemStats>>;

pub fn init_stats() -> SharedSysStats {
    Arc::new(Mutex::new(SystemStats {
        // Timing
        last_activity: None,
        idle_since: None,

        // Keyboard
        last_key_time: None,
        keyboard_burst_start: None,
        keys_in_burst: 0,

        // Mouse
        mouse_moved: false,
        mouse_clicked: false,

        // Scroll wheel
        scroll_active: false,

        // Window context
        focused_window: None,
        last_window_change: None,
    }))
}
