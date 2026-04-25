use std::time::{Duration, Instant};
use std::fs;

use evdev::{Device, InputEvent, EventType, EventSummary::Key};
use procfs::process::all_processes;

use crate::sys::keyboard;
use crate::sys::mouse;
use crate::auxillary::state::SYSTEM_INFO;
use crate::sys::windows::window_info::*;

// =========================
// CONFIG
// =========================

static INPUT_POLL_RATE: Duration = Duration::from_millis(10);
static WINDOW_POLL_RATE: Duration = Duration::from_millis(500);

// =========================
// SYSTEM STATE
// =========================

pub struct SystemInfo {
    pub init_sys_time: Option<Instant>,

    // Keyboard
    pub key_count: u32,
    pub backspace_count: u32,
    pub last_keyboard_activity: Option<Instant>,

    // Mouse
    pub last_mouse_move: Option<Instant>,
    pub last_wheel_scroll: Option<Instant>,
    pub scroll_events: u32,
    pub mouse_clicks: u32,
    pub last_mouse_activity: Option<Instant>,

    // Idle
    pub last_activity: Option<Instant>,

    // Windows
    pub current_window: Option<WindowInfo>,
    pub windows: Vec<WindowInfo>,
    pub window_switch_count: u32,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            init_sys_time: None,

            key_count: 0,
            backspace_count: 0,
            last_keyboard_activity: None,

            last_mouse_move: None,
            last_wheel_scroll: None,
            scroll_events: 0,
            mouse_clicks: 0,
            last_mouse_activity: None,

            last_activity: None,

            current_window: None,
            windows: Vec::new(),
            window_switch_count: 0,
        }
    }

    pub fn check_is_min(&mut self) {
        let now = Instant::now();
        let last = self.init_sys_time.get_or_insert(now);

        if now.duration_since(*last) >= Duration::from_secs(60) {
            self.key_count = 0;
            self.backspace_count = 0;
            self.scroll_events = 0;
            self.mouse_clicks = 0;
            self.window_switch_count = 0;

            *last = now;
        }
    }
}

// =========================
// INPUT SYSTEM (EVDEV)
// =========================

pub fn poll_input_devices() {
    let mut sys = SYSTEM_INFO.lock().unwrap();

    let paths = match fs::read_dir("/dev/input/by-id") {
        Ok(p) => p,
        Err(_) => return,
    };

    for entry in paths.flatten() {
        let path = entry.path();

        let Ok(mut device) = Device::open(&path) else {
            continue;
        };

        if let Ok(events) = device.fetch_events() {
            for ev in events {

                let code: u16 = ev.code();
                let value: i32 = ev.value();

                // =========================
                // KEYBOARD
                // =========================

                match code {

                    14 => { // backspace
                        keyboard::handle_backspace(&mut sys);
                    }

                    272 | 273 | 274 => { // mouse buttons
                        mouse::handle_button_press(&mut sys);
                    }

                    _ => {
                        if value == 1 {
                            keyboard::handle_key_press(&mut sys);
                        }
                    }
                }

                // =========================
                // MOUSE MOVEMENT
                // =========================
                if value != 0 {
                    mouse::handle_mouse_move(&mut sys);
                }
            }
        }
    }
}
// =========================
// WINDOW TRACKING (PID-BASED)
// =========================

pub fn track_window_info() {
    let mut sys = SYSTEM_INFO.lock().unwrap();

    let mut active_proc = None;

    for proc in all_processes().unwrap().flatten() {
        if let Ok(stat) = proc.stat() {
            if stat.state == 'R' {
                active_proc = Some((proc.pid, stat.comm));
                break;
            }
        }
    }

    let Some((pid, name)) = active_proc else {
        return;
    };

    let new_id = pid.to_string();
    let prev = sys.current_window.as_ref().map(|w| w.id.clone());

    // update existing
    if let Some(idx) = sys.windows.iter().position(|w| w.id == new_id) {
        {
            let window = &mut sys.windows[idx];
            window.update_timestamp();
        }

        if prev.as_ref() != Some(&new_id) {
            sys.window_switch_count += 1;
        }

        sys.current_window = sys.windows.get(idx).cloned();
        return;
    }

    // new window
    let window = WindowInfo::new(new_id.clone(), &name, "");
    sys.windows.push(window.clone());
    sys.current_window = Some(window);

    if prev.as_ref() != Some(&new_id) {
        sys.window_switch_count += 1;
    }
}

// =========================
// BACKGROUND LOOP STARTERS
// =========================

pub fn start_input_loop() {
    std::thread::spawn(|| loop {
        poll_input_devices();
        std::thread::sleep(INPUT_POLL_RATE);
    });
}

pub fn start_window_loop() {
    std::thread::spawn(|| loop {
        track_window_info();
        std::thread::sleep(WINDOW_POLL_RATE);
    });
}