// system.rs

use std::time::Instant;

pub struct SystemInfo {
    // Keyboard Tracking
    pub kps: i16,
    pub norm_kps: f32,
    
    // Backspace
    pub backspace_count: i16,
    pub backspace_ratio: f32,
    pub norm_backspace: f32,
    pub backspace_score: f32,

    // Mouse Tracking
    pub last_mouse_move: Option<Instant>,
    pub last_wheel_scroll: Option<Instant>,

    // Idle
    pub last_activity: Option<Instant>,
    pub idle_seconds: i8,
    pub idle_ratio: f32,
    pub idle_score: f32,

    // Window Switching
    pub switch_rate: i16,
    pub norm_switch: f32,
    pub switch_score: f32,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            // Keyboard Tracking
            kps: 0,
            norm_kps: 0.0,

            // Backspace
            backspace_count: 0,
            backspace_ratio: 0.0,
            norm_backspace: 0.0,
            backspace_score: 0.0,

            // Mouse Tracking
            last_mouse_move: None,
            last_wheel_scroll: None,

            // Idle
            last_activity: None,
            idle_seconds: 0,
            idle_ratio: 0.0,
            idle_score: 0.0,

            // Window Switching
            switch_rate: 0,
            norm_switch: 0.0,
            switch_score: 0.0,
        }
    }
}
