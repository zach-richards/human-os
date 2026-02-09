// system.rs

use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

use rdev::{Event, EventType, listen, ListenError};
use once_cell::sync::Lazy;

use crate::sys::mouse;
use crate::sys::keyboard;
use crate::sys::timeout_thread;

pub struct SystemInfo {
    // Timing
    //pub last_activity: Option<Instant>,
    //pub idle_since: Option<Instant>,

    // Keyboard
    pub last_key_time: Option<Instant>,
    pub burst_start: Option<Instant>,
    pub is_burst: bool,
    pub keys_in_burst: u32,

    // Mouse
    pub last_mouse_move: Option<Instant>,
    pub last_mouse_press: Option<Instant>,

    // Scroll wheel
    pub last_wheel_move: Option<Instant>,

    // Window context
    //pub focused_window: Option<String>,
    //pub last_window_change: Option<Instant>,    
}

// create global variable to share across the system
pub static SYSTEM_INFO: Lazy<Arc<Mutex<SystemInfo>>> = 
    Lazy::new(|| Arc::new(Mutex::new(SystemInfo::new())));

fn handle_event(event: Event) {
    const THROTTLE: Duration = Duration::from_millis(50);

    if let Ok(mut sys_info) = SYSTEM_INFO.lock() {
        match event.event_type {
            EventType::KeyPress(_) => {
                keyboard::handle_key_press(&mut *sys_info);
            }

            EventType::ButtonPress(_) => {
                mouse::handle_mouse_press(&mut *sys_info);
            }
            EventType::MouseMove {..} => {
                if sys_info
                    .last_mouse_move
                    .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                {
                    sys_info.last_mouse_move = Some(Instant::now());
                    mouse::handle_mouse_move(&mut *sys_info);
                }
            }
            EventType::Wheel {..} => {
                if sys_info
                    .last_wheel_move
                    .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                {
                    sys_info.last_wheel_move = Some(Instant::now());
                    mouse::handle_wheel(&mut *sys_info);
                }
            }
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
    timeout_thread::timeout_thread();
    listen(move |event: Event| {handle_event(event)})
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            // Timing
            //last_activity: None,
            //idle_since: None,

            // Keyboard
            last_key_time: None,
            burst_start: None,
            is_burst: false,
            keys_in_burst: 0,

            // Mouse
            last_mouse_move: None,
            last_mouse_press: None,

            // Scroll wheel
            last_wheel_move: None,

            // Window context
            //focused_window: None,
            //last_window_change: None,
        }
    }
}
