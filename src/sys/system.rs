// system.rs

use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

use rdev::{Event, EventType, listen, ListenError};
use once_cell::sync::Lazy;

use crate::sys::keyboard;

pub struct SystemInfo {
    // Timing
    pub last_activity: Option<Instant>,
    pub idle_since: Option<Instant>,

    // Keyboard
    pub last_key_time: Option<Instant>,
    pub burst_start: Option<Instant>,
    pub keys_in_burst: u32,

    // Mouse
    pub mouse_moved: bool,
    pub mouse_clicked: bool,

    // Scroll wheel
    pub scroll_active: bool,

    // Window context
    pub focused_window: Option<String>,
    pub last_window_change: Option<Instant>,    
}

// create global variable to share across the system
pub static SYSTEM_INFO: Lazy<Arc<Mutex<SystemInfo>>> = 
    Lazy::new(|| Arc::new(Mutex::new(SystemInfo::new())));

const THROTTLE: Duration = Duration::from_millis(50);

fn handle_event(event: Event) {
    if let Ok(mut sys) = SYSTEM_INFO.lock() {
        match event.event_type {
            EventType::KeyPress(_) => {
                keyboard::handle_key_press(&mut *sys);
            }

            /* Some(EventType::MousePress(_)) {
                mouse.button_active();
            }
            Some(EventType::MouseMove {..} && last_mouse_move > THROTTLE) {
                mouse.movement_active();
            }
            Some(EventType::Wheel {..} && last_wheel_move > THROTTLE) {
                mouse.wheel_active();
            } */
            // window active
            // internet tab active
            // window/app name
            // tab title
            // activity/focus timing
            // Ex. Steam after Word ("Return focus to writing/studying?")

            _ => { /* ignore other cases */ }
        }
    }
}

pub fn track_system() -> Result<(), ListenError>  {
    listen(handle_event)
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            // Timing
            last_activity: None,
            idle_since: None,

            // Keyboard
            last_key_time: None,
            burst_start: None,
            keys_in_burst: 0,

            // Mouse
            mouse_moved: false,
            mouse_clicked: false,

            // Scroll wheel
            scroll_active: false,

            // Window context
            focused_window: None,
            last_window_change: None,
        }
    }
}
