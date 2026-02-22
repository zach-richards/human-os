// main.rs

mod sys;
mod logic;

use std::time::{ Instant, Duration };
use std::sync::{ Arc, Mutex };
use std::thread;

use rdev::{ listen,  ListenError };
use once_cell::sync::Lazy;

use crate::logic::cognitive_model;
use crate::sys::system;
use crate::sys::window;

// create global variable to share across the system
static SYSTEM_INFO: Lazy<Arc<Mutex<system::SystemInfo>>> =
    Lazy::new(|| Arc::new(Mutex::new(system::SystemInfo::new())));

static COGNITIVE_MODEL: Lazy<Arc<Mutex<cognitive_model::CognitiveModel>>> =
    Lazy::new(|| Arc::new(Mutex::new(cognitive_model::CognitiveModel::new())));

fn main() -> Result<(), ListenError> {
    println!("  DEBUG LOG");
    println!("--------------");

    SYSTEM_INFO.lock().unwrap().init_sys_time = Some(Instant::now());

    thread::spawn(move || {
        
        listen(system::handle_input_event).unwrap();
        thread::yield_now();
        
    });

    // track window switches in different thread
    thread::spawn(move || {
        let sys_info_clone = &SYSTEM_INFO;
        window::track_window_switches(sys_info_clone).unwrap();
    });

    loop {
        {
            let mut cog_model_clone = COGNITIVE_MODEL.lock().unwrap();
            let mut sys_info_clone = SYSTEM_INFO.lock().unwrap();

            cog_model_clone.update(&sys_info_clone);
            sys_info_clone.check_is_min();
            cog_model_clone.print();
        }

        thread::sleep(Duration::from_secs(1));
    }
}
