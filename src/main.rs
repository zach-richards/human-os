// main.rs

mod sys;

use std::time::{ Duration, Instant };
use std::sync::{ Arc, Mutex };
use std::thread;

use rdev::{ listen, Event, EventType, ListenError, Key };
use once_cell::sync::Lazy;

use crate::sys::mouse;
use crate::sys::system;
use crate::sys::window;
use crate::sys::keyboard;

// create global variable to share across the system
static SYSTEM_INFO: Lazy<Arc<Mutex<sys::system::SystemInfo>>> =
    Lazy::new(|| Arc::new(Mutex::new(system::SystemInfo::new())));

static THROTTLE: Duration = Duration::from_millis(100);

fn handle_event(event: Event) {
    let mut mut_sys_info = SYSTEM_INFO.lock().unwrap();

    // track keyboard, mouse, and mouse buttons in seperate thread
    match event.event_type {
        EventType::KeyPress(Key::Backspace) => {
            keyboard::handle_backspace(&mut mut_sys_info);
        }

        EventType::KeyPress(_) => {
            keyboard::handle_key_press(&mut mut_sys_info);
        }

        EventType::ButtonPress(_) => {
            mouse::handle_button_press(&mut mut_sys_info);
        }

        EventType::MouseMove {..} => {
            if mut_sys_info
                .last_mouse_move
                .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
            {
                mouse::handle_mouse_move(&mut mut_sys_info);
            }
        }

        EventType::Wheel {..} => {
            if mut_sys_info
                .last_wheel_scroll
                .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
            {
                mouse::handle_wheel_scroll(&mut mut_sys_info);

            }

        }

        _ => { /* ignore */ }
    }
}

fn main() -> Result<(), ListenError> {
    #[cfg(debug_assertions)]
    println!("  DEBUG LOG");
    println!("--------------");

    thread::spawn(move || {
        
        listen(handle_event).unwrap();
        thread::yield_now();
        
    });

    // track window switches in different thread
    thread::spawn(move || {
        let sys_info_clone = &SYSTEM_INFO;
        window::track_window_switches(sys_info_clone).unwrap();
    });

    loop {}
}
