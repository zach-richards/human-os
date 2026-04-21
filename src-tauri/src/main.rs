mod sys;
mod logic;
mod notifications;

use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::thread;

use once_cell::sync::Lazy;
use rdev::listen;

use crate::logic::cognitive_model;
use crate::sys::system;

// =========================
// GLOBAL STATE
// =========================

static SYSTEM_INFO: Lazy<Arc<Mutex<system::SystemInfo>>> =
    Lazy::new(|| Arc::new(Mutex::new(system::SystemInfo::new())));

static COGNITIVE_MODEL: Lazy<Arc<Mutex<cognitive_model::CognitiveModel>>> =
    Lazy::new(|| Arc::new(Mutex::new(cognitive_model::CognitiveModel::new())));

// =========================
// SYSTEM INIT
// =========================

fn initialize_system_time() {
    SYSTEM_INFO
        .lock()
        .unwrap()
        .init_sys_time = Some(Instant::now());
}

// =========================
// INPUT TRACKING
// =========================

fn start_system_input_update_loop() {
    thread::spawn(|| {
        listen(system::handle_input_event).unwrap();
    });
}

// =========================
// WINDOW TRACKING
// =========================

fn start_window_info_update_loop() {
    thread::spawn(|| {
        loop {
            system::track_window_info();
            thread::sleep(Duration::from_millis(100));
        }
    });
}

// =========================
// COGNITIVE MODEL LOOP
// =========================

fn start_cognitive_loop() {
    thread::spawn(|| {
        loop {
            {
                let mut model = COGNITIVE_MODEL.lock().unwrap();
                let sys = SYSTEM_INFO.lock().unwrap();

                model.update(&sys);
                sys.check_is_min();
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}

// =========================
// DECISION ENGINE LOOP
// =========================

fn start_decision_engine_loop() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(30));

            let (key_count, backspace_count, window_switch_count, idle_secs) = {
                let sys = SYSTEM_INFO.lock().unwrap();

                (
                    sys.key_count,
                    sys.backspace_count,
                    sys.window_switch_count,
                    sys.last_activity
                        .map(|t| Instant::now().duration_since(t).as_secs() as i16)
                        .unwrap_or(0),
                )
            };

            logic::decision_eng::run(
                key_count,
                backspace_count,
                window_switch_count,
                idle_secs,
            );
        }
    });
}

// =========================
// CORE ENGINE START
// =========================

pub fn run_engine() {
    initialize_system_time();

    start_system_input_update_loop();
    start_window_info_update_loop();
    start_cognitive_loop();
    start_decision_engine_loop();

    // IMPORTANT:
    // No GTK, no blocking GUI loop.
    // Tauri handles UI lifetime.
}

// =========================
// MAIN ENTRY FOR DAEMON MODE
// =========================

fn main() {
    run_engine();

    // If running as standalone daemon:
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}