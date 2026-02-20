// main.rs

mod sys;

use std::sync::{Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use rdev::{listen, Event, EventType, Key};
use once_cell::sync::Lazy;

use crate::sys::keyboard;
use crate::sys::mouse;
use crate::sys::system::SystemInfo;
use crate::sys::window;

static GLOBAL_TX: Lazy<Mutex<Option<mpsc::Sender<SystemEvent>>>> =
    Lazy::new(|| Mutex::new(None));

// Throttle for mouse movement & wheel
static THROTTLE: Duration = Duration::from_millis(50);

/// Unified application event
enum SystemEvent {
    Input(Event),
    WindowChanged(String),
}

fn event_callback(event: Event) {
    if let Some(tx) = &*GLOBAL_TX.lock().unwrap() {
        tx.send(SystemEvent::Input(event)).unwrap();
    }
}

fn main() {
    #[cfg(debug_assertions)]
    println!("DEBUG LOG\n--------------");

    let (tx, rx) = mpsc::channel::<SystemEvent>();

    // ==========================
    // Listener Thread
    // ==========================
    let tx_input = tx.clone();
    thread::spawn(move || {
        listen(move |event| { event_callback(event)}).unwrap();
    });

    // ==========================
    // Window Tracker Thread
    // ==========================
    let tx_window = tx.clone();
    thread::spawn(move || {
        loop {
            if let Ok(active_window) = window::get_active_window() {
                tx_window.send(SystemEvent::WindowChanged(active_window)).unwrap();
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // ==========================
    // Worker Thread (Owns SystemInfo)
    // ==========================
    thread::spawn(move || {
        let mut sys_info = SystemInfo::new();

        while let Ok(app_event) = rx.recv() {
            match app_event {
                SystemEvent::Input(event) => {
                    match event.event_type {

                        EventType::KeyPress(Key::Backspace) => {
                            keyboard::handle_backspace(&mut sys_info);
                            println!("Backspace pressed!");
                        }

                        EventType::KeyPress(_) => {
                            keyboard::handle_key_press(&mut sys_info);
                        }

                        EventType::ButtonPress(_) => {
                            mouse::handle_button_press(&mut sys_info);
                        }

                        EventType::MouseMove { .. } => {
                            if sys_info.last_mouse_move
                                .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                            {
                                mouse::handle_mouse_move(&mut sys_info);
                            }
                        }

                        EventType::Wheel { .. } => {
                            if sys_info.last_wheel_scroll
                                .map_or(true, |t| Instant::now().duration_since(t) >= THROTTLE)
                            {
                                mouse::handle_wheel_scroll(&mut sys_info);
                            }
                        }

                        _ => {}
                    }
                }

                SystemEvent::WindowChanged(new_window) => {
                    if Some(&new_window) != sys_info.current_window.as_ref() {
                        sys_info.current_window = Some(new_window);
                        sys_info.switch_rate += 1;
                        println!("Switched window!");
                    }
                }
            }
        }
    });

    // Keep main alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
