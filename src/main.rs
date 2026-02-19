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

static THROTTLE: Duration = Duration::from_millis(50);

fn main() -> Result<(), ListenError> {
    #[cfg(debug_assertions)]
    println!("  DEBUG LOG");
    println!("--------------");

    thread::spawn(move || {

        listen(move |event: Event| {
            println!("Hello thread");
        
            // track keyboard, mouse, and mouse buttons in seperate thread
            if let Ok(mut sys_info) = SYSTEM_INFO.lock() {
                println!("Hello hello");
                match event.event_type {
                    EventType::KeyPress(Key::Backspace) => {
                        keyboard::handle_backspace(&mut sys_info);
                    }

                    EventType::KeyPress(_) => {
                        keyboard::handle_key_press(&mut *sys_info);
                    }

                    EventType::ButtonPress(_) => {
                        mouse::handle_button_press(&mut *sys_info);
                    }

                    EventType::MouseMove {..} => {
                        if sys_info
                            .last_mouse_move
                            .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                        {
                            mouse::handle_mouse_move(&mut *sys_info);
                        }
                    }

                    EventType::Wheel {..} => {
                        if sys_info
                            .last_wheel_scroll
                            .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                        {
                            mouse::handle_wheel_scroll(&mut *sys_info);
                        }
                        
                    }

                    _ => { /* ignore */ }
                } 
            
            }
        }).unwrap();
        
    });

    // track window switches in different thread
    thread::spawn(move || {

        if let Ok(mut sys_info) = SYSTEM_INFO.lock() {
            window::track_window_switches(&mut *sys_info).unwrap();
        }
        
    });

    loop {}
}
