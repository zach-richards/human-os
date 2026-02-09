// timeout_thread.rs

use std::time::{Instant, Duration};
use std::thread;
use std::sync::Arc;

use crate::sys::system::SYSTEM_INFO;

pub fn timeout_thread() {
    const BURST_TIMEOUT: Duration = Duration::from_secs(2);

    let sys_info_clone = Arc::clone(&SYSTEM_INFO);
    thread::spawn(move || loop {
        {
            let now = Instant::now();

            let mut sys_info = sys_info_clone.lock().unwrap();
            if sys_info.is_burst == true {
                if let Some(last) = sys_info.last_key_time {
                    if Instant::now().duration_since(last) > BURST_TIMEOUT {
                             if let Some(start) = sys_info.burst_start {
                             let duration = now.duration_since(start).as_secs_f64();
                             let kps = sys_info.keys_in_burst as f64 / duration;
                             println!(
                                "\n\nBurst ended: {} keys in {:.2} s -> {:.2} KPS\n",
                                sys_info.keys_in_burst, duration, kps
                             );
                             sys_info.burst_start = None; // ✅ reset so it only prints once
                             sys_info.is_burst = false;   // also reset burst
                             sys_info.keys_in_burst = 0;
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
