// mouse.rs

use std::time::Instant;

use crate::sys::system;

pub fn handle_button_press(sys_info: &mut system::SystemInfo) {
    sys_info.last_activity = Some(Instant::now());
    println!("Button pressed!");
}

pub fn handle_mouse_move(sys_info: &mut system::SystemInfo) {
    sys_info.last_activity = Some(Instant::now());
    sys_info.last_mouse_move = sys_info.last_activity;
    println!("Mouse moved!");
}

pub fn handle_wheel_scroll(sys_info: &mut system::SystemInfo) {
    sys_info.last_activity = Some(Instant::now());
    sys_info.last_wheel_scroll = sys_info.last_activity;
    println!("Wheel scroll!");
}
