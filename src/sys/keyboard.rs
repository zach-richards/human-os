// keyboard.rs

use std::time::{Instant, Duration};

use crate::sys::system::SystemInfo;

pub fn handle_key_press(system_info_w: &mut SystemInfo) {
    let now = Instant::now();
    const BURST_TIMEOUT: Duration = Duration::from_secs(2);

    match system_info_w.last_key_time {
        Some(last) if now.duration_since(last) > BURST_TIMEOUT => {
            if let Some(start) = system_info_w.burst_start {
                let duration = now.duration_since(start).as_secs_f64();
                let kps = system_info_w.keys_in_burst as f64 / duration;
                println!(
                    "Burst ended: {} keys in {:.2} s -> {:.2} KPS",
                    system_info_w.keys_in_burst, duration, kps
                );
            }
            system_info_w.burst_start = Some(now);
            system_info_w.keys_in_burst = 1;
        }
        Some(_) => system_info_w.keys_in_burst += 1,
        None => {
            system_info_w.burst_start = Some(now);
            system_info_w.keys_in_burst = 1;
        }
    }

    system_info_w.last_key_time = Some(now);
}

