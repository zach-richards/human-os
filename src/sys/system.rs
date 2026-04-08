// system.rs

// struct to hold system info and update info.

use std::time::{ Duration, Instant };

use rdev::{ Event, EventType, Key };

use crate::sys::mouse;
use crate::sys::keyboard;
use crate::SYSTEM_INFO;
use crate::sys::windows::window_info::WindowInfo;

static THROTTLE: Duration = Duration::from_millis(100);

pub struct SystemInfo {
    // Track mins
    pub init_sys_time: Option<Instant>,

    // Keyboard tracking
    pub key_count: i16,
    
    // Backspace
    pub backspace_count: i16,

    // Mouse tracking
    pub last_mouse_move: Option<Instant>,
    pub last_wheel_scroll: Option<Instant>,

    // Idle
    pub last_activity: Option<Instant>,

    // Windows
    pub windows: Vec<WindowInfo>,
    pub window_switch_count: i16,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            // Track mins
            init_sys_time: None,

            // Keyboard tracking
            key_count: 0,

            // Backspace
            backspace_count: 0,

            // Mouse tracking
            last_mouse_move: None,
            last_wheel_scroll: None,

            // Idle
            last_activity: None,

            // Windows
            windows: Vec::new(),
            window_switch_count: 0,
        }
    }

    pub fn check_is_min(&mut self) {
        let now = Instant::now();

        let last_reset = self.init_sys_time.get_or_insert(now);

        let elapsed = now.duration_since(*last_reset);

        // Check if we've crossed a full minute
        if elapsed.as_secs() >= 60 {
            self.key_count = 0;
            self.backspace_count = 0;
            self.window_switch_count = 0;

            let minutes_elapsed = elapsed.as_secs() / 60;
            *last_reset += Duration::from_secs(minutes_elapsed * 60);
        }
    }
    
    /*
    pub fn print(&self) {
        println!("Initial System Time: {:?}", self.init_sys_time);
        println!("Key Count: {}", self.key_count);
        println!("Backspace Count: {}", self.backspace_count);
        println!("Last Mouse Move: {:?}", self.last_mouse_move);
        println!("Last Wheel Scroll: {:?}", self.last_wheel_scroll);
        println!("Last Activity: {:?}", self.last_activity);
        println!("Window Switch Count: {}\n", self.window_switch_count);
    }
    */
}

impl std::fmt::Debug for SystemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemInfo")
            .field("init_sys_time", &self.init_sys_time)
            .field("key_count", &self.key_count)
            .field("backspace_count", &self.backspace_count)
            .field("last_mouse_move", &self.last_mouse_move)
            .field("last_wheel_scroll", &self.last_wheel_scroll)
            .field("last_activity", &self.last_activity)
            .field("windows_count", &self.windows.len())
            .field("window_switch_count", &self.window_switch_count)
            .finish()
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

pub fn track_window_info() {
    let mut mut_sys_info = SYSTEM_INFO.lock().unwrap();
    let Ok(win) = get_active_window() else {
        return;
    };

    for window in &mut mut_sys_info.windows {
        if window.id == win.window_id {
            window.update_title
            window.update_timestamp();
            return;
        }
     }
    }

    if let Some(existing_window) = mut_sys_info.windows.iter().find(|w| w.id == window_info.id) {
        println!("Window already exists: {:?}", existing_window);
    } else {
        println!("New window detected: {:?}", window_info);
    }
}