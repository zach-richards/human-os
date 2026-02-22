// keyboard.rs

use std::time::Instant;

use crate::sys::system::SystemInfo;

pub fn handle_backspace(sys_info: &mut SystemInfo) {
    sys_info.backspace_count += 1;
    sys_info.last_activity = Some(Instant::now());
}

pub fn handle_key_press(sys_info: &mut SystemInfo) {
    sys_info.key_count += 1;
    sys_info.last_activity = Some(Instant::now());
}

