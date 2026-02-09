// mouse.rs

use std::time::Instant;

use crate::sys::window;
use crate::sys::system::SystemInfo;

pub fn handle_mouse_press(sys_info: &mut SystemInfo) {
    sys_info.last_mouse_press = Some(Instant::now());
    println!("Mouse pressed!");
    window::handle_window_switch(sys_info);
}

pub fn handle_mouse_move(sys_info: &mut SystemInfo) {
    sys_info.last_mouse_move = Some(Instant::now());
    println!("Mouse moved!");
}

pub fn handle_wheel(sys_info: &mut SystemInfo) {
    sys_info.last_wheel_move = Some(Instant::now());
    println!("Wheel scrolled!");
}
