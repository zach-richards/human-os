use rdev::{listen, Event};
use std::time::{Instant, Duration};
use std::sync::Mutex;

#[derive(Debug)]
struct KeyboardStats {
    last_key_time: Option<Instant>,
    burst_start: Option<Instant>,
    keys_in_burst: u32,
}

// Safe global using Mutex
static STATS: Mutex<KeyboardStats> = Mutex::new(KeyboardStats {
    last_key_time: None,
    burst_start: None,
    keys_in_burst: 0,
});

fn handle_event(event: Event) {
    unsafe {
        let mut stats = STATS.lock().unwrap();

        if let rdev::EventType::KeyPress(_) = event.event_type {
            let now = Instant::now();
            const BURST_TIMEOUT: Duration = Duration::from_secs(2);

            match stats.last_key_time {
                Some(last) if now.duration_since(last) > BURST_TIMEOUT => {
                    if let Some(start) = stats.burst_start {
                        let duration = now.duration_since(start).as_secs_f64();
                        let kps = stats.keys_in_burst as f64 / duration;
                        println!(
                            "Burst ended: {} keys in {:.2} s -> {:.2} KPS",
                            stats.keys_in_burst, duration, kps
                        );
                    }
                    stats.burst_start = Some(now);
                    stats.keys_in_burst = 1;
                }
                Some(_) => stats.keys_in_burst += 1,
                None => {
                    stats.burst_start = Some(now);
                    stats.keys_in_burst = 1;
                }
            }

            stats.last_key_time = Some(now);
        }
    }
}

fn main() {
    listen(handle_event).unwrap(); // ✅ plain function pointer
}

