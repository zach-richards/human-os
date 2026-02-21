// system.rs

use std::time::{ Instant, Duration };

use rdev::{ Event, EventType, Key };

use crate::sys::mouse;
use crate::sys::keyboard;
use crate::SYSTEM_INFO;

static THROTTLE: Duration = Duration::from_millis(100);

pub struct SystemInfo {
    // Track Mins
    pub init_sys_time: Option<Instant>,

    // Keyboard Tracking
    pub key_count: i16,
    
    // Backspace
    pub backspace_count: i16,

    // Mouse Tracking
    pub last_mouse_move: Option<Instant>,
    pub last_wheel_scroll: Option<Instant>,

    // Idle
    pub last_activity: Option<Instant>,

    // Window Switching
    pub window_switch_count: i16,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            // Track Mins
            init_sys_time: None,

            // Keyboard Tracking
            key_count: 0,

            // Backspace
            backspace_count: 0,

            // Mouse Tracking
            last_mouse_move: None,
            last_wheel_scroll: None,

            // Idle
            last_activity: None,

            // Window Switching
            window_switch_count: 0,
        }
    }
}

pub fn handle_input_event(event: Event) {
    let mut mut_sys_info = SYSTEM_INFO.lock().unwrap();

    // track keyboard, mouse, and mouse buttons in seperate thread
    match event.event_type {
        EventType::KeyPress(Key::Backspace) => {
            keyboard::handle_backspace(&mut mut_sys_info);
        }

        EventType::KeyPress(_) => {
            keyboard::handle_key_press(&mut mut_sys_info);
        }

        EventType::ButtonPress(_) => {
            mouse::handle_button_press(&mut mut_sys_info);
        }

        EventType::MouseMove {..} => {
            if mut_sys_info
                .last_mouse_move
                .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
            {
                mouse::handle_mouse_move(&mut mut_sys_info);
            }
        }

        EventType::Wheel {..} => {
            if mut_sys_info
                .last_wheel_scroll
                .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
            {
                mouse::handle_wheel_scroll(&mut mut_sys_info);

            }

        }

        _ => { /* ignore */ }
    }
}
