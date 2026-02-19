// main.rs

mod sys;

use std::time::{ Duration, Instant };
use std::sync::{ mpsc, Arc, Mutex };
use std::thread;

use rdev::{ listen, Event, EventType, ListenError, Key };
use once_cell::sync::Lazy;

use crate::sys::mouse;
use crate::sys::system::SystemInfo;
use crate::sys::window;
use crate::sys::keyboard;

// Global Arc<Mutex>
static SYSTEM_INFO: Lazy<Arc<Mutex<SystemInfo>>> = Lazy::new(|| Arc::new(Mutex::new(SystemInfo::new())));

// Global sender for rdev callback
static GLOBAL_TX: Lazy<Mutex<Option<mpsc::Sender<Event>>>> = Lazy::new(|| Mutex::new(None));

static THROTTLE: Duration = Duration::from_millis(50);

fn event_callback(event: Event) {
    // must be a function pointer, cannot capture variables
    if let Some(tx) = &*GLOBAL_TX.lock().unwrap() {
        tx.send(event).unwrap();
    }
}

fn main() {
    #[cfg(debug_assertions)]
    println!("  DEBUG LOG");
    println!("--------------");

    let (tx, rx) = mpsc::channel();

    // store sender globally
    *GLOBAL_TX.lock().unwrap() = Some(tx);

    // listener thread
    thread::spawn(|| {
        listen(event_callback).unwrap();
    });

    // worker thread
    let sys_info_clone = SYSTEM_INFO.clone();
    thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            let mut sys_info = sys_info_clone.lock().unwrap();

            match event.event_type {
                EventType::KeyPress(Key::Backspace) => {
                    keyboard::handle_backspace(&mut sys_info);
                    println!("Backspace pressed!");
                }
                EventType::KeyPress(_) => keyboard::handle_key_press(&mut sys_info),
                EventType::ButtonPress(_) => mouse::handle_button_press(&mut sys_info),
                EventType::MouseMove { .. } => {
                    if sys_info.last_mouse_move.map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                    {
                        mouse::handle_mouse_move(&mut sys_info);
                    }
                }
                EventType::Wheel { .. } => {
                    if sys_info.last_wheel_scroll.map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                    {
                        mouse::handle_wheel_scroll(&mut sys_info);
                    }
                }
                _ => {}
            }
        }
    });
/*
    // track window switches in different thread
    thread::spawn(move || {

        if let Ok(mut sys_info) = SYSTEM_INFO.lock() {
            window::track_window_switches(&mut *sys_info).unwrap();
        }

    });
*/
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
