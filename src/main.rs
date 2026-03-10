// main.rs

mod sys;
mod logic;

use std::time::{ Instant, Duration };
use std::sync::{ Arc, Mutex };
use std::thread;

use gtk::prelude::*;
use tray_icon::{TrayIconBuilder, menu::Menu, icon::Icon};
use rdev::{ listen,  ListenError };
use once_cell::sync::Lazy;

use crate::logic::cognitive_model;
use crate::sys::system;
use crate::sys::window;

// create global variable to share across the system
static SYSTEM_INFO: Lazy<Arc<Mutex<system::SystemInfo>>> =
    Lazy::new(|| Arc::new(Mutex::new(system::SystemInfo::new())));

// holds cognitive focus score
static COGNITIVE_MODEL: Lazy<Arc<Mutex<cognitive_model::CognitiveModel>>> =
    Lazy::new(|| Arc::new(Mutex::new(cognitive_model::CognitiveModel::new())));

<<<<<<< HEAD
fn main() -> Result<(), ListenError> {
    #[cfg(debug_assertions)]
=======
fn main() {
    gtk::init().expect("Failed to initialize GTK");
    
    let width = 16;
    let height = 16;

    // Create RGBA buffer
    let mut rgba = Vec::new();

    for _ in 0..(width * height) {
        rgba.extend_from_slice(&[255, 0, 0, 255]); // Red, fully opaque
    }

    let icon = Icon::from_rgba(rgba, width, height).unwrap();
    let tray_menu = Menu::new();
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();

>>>>>>> cfb2e8a (Started implementing rgba tray-icon)
    println!("  DEBUG LOG");
    println!("--------------");

    SYSTEM_INFO.lock().unwrap().init_sys_time = Some(Instant::now());

    // a thread to handle input event loop
    thread::spawn(move || {
        listen(system::handle_input_event).unwrap();
        thread::yield_now();
    });

    // track window switches in different thread
    thread::spawn(move || {
        let sys_info_clone = &SYSTEM_INFO;
        window::track_window_switches(sys_info_clone).unwrap();
    });

<<<<<<< HEAD
    // thread to update cog model and sys info
    loop {
        {
            let mut cog_model_clone = COGNITIVE_MODEL.lock().unwrap();
            let mut sys_info_clone = SYSTEM_INFO.lock().unwrap();
=======
    thread::spawn(|| {
        loop {
            {
                let mut cog_model_clone = COGNITIVE_MODEL.lock().unwrap();
                let mut sys_info_clone = SYSTEM_INFO.lock().unwrap();
>>>>>>> cfb2e8a (Started implementing rgba tray-icon)

                cog_model_clone.update(&sys_info_clone);
                sys_info_clone.check_is_min();
                cog_model_clone.print();
            }

            thread::sleep(Duration::from_secs(1));
        }
    });

    gtk::main();
}
