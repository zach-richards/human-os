// system.rs

use std::time::Instant;

pub struct SystemInfo {
    // Keyboard Tracking
    pub kps: i16,
    
    // Backspace
    pub backspace_count: i16,

    // Mouse Tracking
    pub last_mouse_move: Option<Instant>,
    pub last_wheel_scroll: Option<Instant>,

    // Idle
    pub last_activity: Option<Instant>,
    pub idle_seconds: i8,

    // Window Switching
    pub switch_rate: i16,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            // Keyboard Tracking
            kps: 0,

            // Backspace
            backspace_count: 0,

            // Mouse Tracking
            last_mouse_move: None,
            last_wheel_scroll: None,

            // Idle
            last_activity: None,
            idle_seconds: 0,

            // Window Switching
            switch_rate: 0,
        }
    }
}
