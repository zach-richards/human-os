// keyboard.rs
// Get keyboard type speed, idle time
// Typing burst: 47 keys / 20 secs

/* This file counts the number of keys in a burst. A burst is keys pressed in a row without stopping
 * for 2 seconds. */

use rdev::{listen, Event, EventType};
use std::time::{Instant, Duration};

use crate::system::sys_stats::SharedSysStats;

fn active(sys_stats: SharedSysStats) {
    // Listen for keys
    listen(move |event: Event| {
        if let EventType::KeyPress(_key) = event.event_type {
            let now = Instant::now();

            const BURST_TIMEOUT: Duration = Duration::from_secs(2);

            match SharedSysStats.last_key_time {

                Some(last) if now.duration_since(last) > BURST_TIMEOUT => {
                    // Previous burst end
                    if let Some(start) = SharedSysStats.burst_start {
                        let duration = now.duration_since(start).as_secs_f64();
                        let kps = SharedSysStats.keys_in_burst as f64 / duration;
                        println!("Burst ended: {} keys in {:.2} s -> {:.2} KPS",
                            SharedSysStats.keys_in_burst, duration, kps);
                    }
                    // Start new burst
                    SharedSysStats.burst_start = Some(now);
                    SharedSysStats.keys_in_burst = 1;
                }
                Some(_) => {
                    SharedSysStats.keys_in_burst += 1;
                }
                None => {
                    SharedSysStats.burst_start = Some(now);
                    SharedSysStats.keys_in_burst = 1;
                }
            }

            SharedSysStats.last_key_time = Some(now);
        }
    }).unwrap();
}
