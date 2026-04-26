// engine.rs

// An abstract handling of all the threads and tracking

use std::time::{Instant, Duration};
use std::thread;

use rdev::listen;

use crate::logic::decision_eng;
use crate::logic::intervention::trigger_intervention;
use crate::sys::system;
use crate::auxillary::state::{SYSTEM_INFO, COGNITIVE_MODEL};
use crate::ui::tray_icon;
use crate::communication::rs_sender;

fn initialize_system_time() {
    SYSTEM_INFO
        .lock()
        .unwrap()
        .init_sys_time = Some(Instant::now());
}

fn start_system_input_update_loop() {
    thread::spawn(|| {
        listen(system::handle_input_event).unwrap();
    });
}

fn start_window_info_update_loop() {
    thread::spawn(|| {
        loop {
            system::track_window_info();
            thread::sleep(Duration::from_millis(100));
        }
    });
}

fn start_cognitive_loop() {
    thread::spawn(|| {
        loop {
            {
                let mut model = COGNITIVE_MODEL.lock().unwrap();
                let mut sys = SYSTEM_INFO.lock().unwrap();

                model.update(&sys);
                sys.check_is_min();
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn start_decision_engine_loop() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(15));

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

            let intervention = decision_eng::run(
                key_count,
                backspace_count,
                window_switch_count,
                idle_secs,
            );

            trigger_intervention(intervention);
        }
    });
}

pub fn start_ui_loop(app: &tauri::AppHandle) {
    let app = app.clone();

    thread::spawn(move || {
        loop {
            let score = {
                let cog_model = COGNITIVE_MODEL.lock().unwrap();
                cog_model.score
            };

            tray_icon::update_focus_fuel(&app, score).unwrap();
            rs_sender::send_focus_update(&app, score);

            thread::sleep(Duration::from_secs(1));
        }
    });
}

pub fn run_engine(app: &tauri::AppHandle) {
    initialize_system_time();

    start_system_input_update_loop();
    start_window_info_update_loop();
    start_cognitive_loop();
    start_ui_loop(&app);
    start_decision_engine_loop();
}
