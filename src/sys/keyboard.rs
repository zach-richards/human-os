// keyboard.rs

use std::time::Instant;

use crate::sys::system::SystemInfo;

pub fn handle_key_press(sys_info: &mut SystemInfo) {

    let now = Instant::now();

    sys_info.last_key_time = Some(now);

    match sys_info.last_key_time {
        Some(_) if sys_info.is_burst == false => {
            sys_info.is_burst = true;
            sys_info.burst_start = Some(now);
            sys_info.keys_in_burst = 1;
        }
        Some(_) => sys_info.keys_in_burst += 1,
        None => {
            sys_info.burst_start = Some(now);
            sys_info.keys_in_burst = 1;
        }
    }

    sys_info.last_key_time = Some(now);
}

