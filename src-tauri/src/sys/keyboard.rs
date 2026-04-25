use std::time::Instant;
use crate::sys::system::SystemInfo;

pub fn handle_backspace(sys: &mut SystemInfo) {
    sys.key_count += 1;
    sys.backspace_count += 1;

    // keyboard is "hard activity"
    sys.last_keyboard_activity = Some(Instant::now());
    sys.last_activity = Some(Instant::now());
}

pub fn handle_key_press(sys: &mut SystemInfo) {
    sys.key_count += 1;

    sys.last_keyboard_activity = Some(Instant::now());
    sys.last_activity = Some(Instant::now());
}