// main.rs

mod sys;
mod logic;
mod ui;

use std::time::{ Instant, Duration };
use std::sync::{ Arc, Mutex };
use std::thread;

use rdev::listen;
use once_cell::sync::Lazy;
use gtk::glib;

use crate::ui::tray_icon::TrayIcon;
use crate::logic::cognitive_model;
use crate::sys::system;
use crate::sys::windows::window;

// create global variable to share across the system
static SYSTEM_INFO: Lazy<Arc<Mutex<system::SystemInfo>>> =
    Lazy::new(|| Arc::new(Mutex::new(system::SystemInfo::new())));

// holds cognitive focus score
static COGNITIVE_MODEL: Lazy<Arc<Mutex<cognitive_model::CognitiveModel>>> =
    Lazy::new(|| Arc::new(Mutex::new(cognitive_model::CognitiveModel::new())));

fn initialize_system_time() {
    let mut sys_info = SYSTEM_INFO.lock().unwrap();
    sys_info.init_sys_time = Some(Instant::now());
}

fn start_system_info_update_loop() {
    thread::spawn(move || {
        listen(system::handle_input_event).unwrap();
        listen(system::track_window_info(window_info));
        thread::yield_now();
    });
}

fn start_listening_for_window_switches_loop() {
    thread::spawn(move || {
        let sys_info_clone = &SYSTEM_INFO; // start_listening_for_window_switches_loop()
        window::track_window_switches(sys_info_clone).unwrap();
    });
}

fn start_cog_model_and_sys_info_update_loop() {
    thread::spawn(move || {
        loop {
            {
            let mut cog_model_clone = COGNITIVE_MODEL.lock().unwrap();
            let mut sys_info_clone = SYSTEM_INFO.lock().unwrap();

                cog_model_clone.update(&sys_info_clone);
                sys_info_clone.check_is_min();
                // cog_model_clone.print();
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn initialize_tray_icon() -> TrayIcon {
    let tray = TrayIcon::new();
    {
        let mut cog_model_clone = COGNITIVE_MODEL.lock().unwrap(); // initialize_tray_icon()
        let sys_info_clone = SYSTEM_INFO.lock().unwrap();
        cog_model_clone.update(&sys_info_clone);
        
        tray.setup(cog_model_clone.score);
    }
    tray
}

fn start_tray_icon_update_loop(tray: TrayIcon) {
    glib::timeout_add_local(Duration::from_secs(2), move || {
        let mut cog_model_clone = COGNITIVE_MODEL.lock().unwrap();
        let sys_info_clone = SYSTEM_INFO.lock().unwrap();

        cog_model_clone.update(&sys_info_clone);
        tray.run(cog_model_clone.score);

        glib::Continue(true)
    });
}

fn main() -> Result<(), rdev::ListenError> {
    initialize_system_time();

    start_system_info_update_loop();

    start_listening_for_window_switches_loop();

    start_cog_model_and_sys_info_update_loop();

    // Initialize GTK for tray icon
    gtk::init().unwrap();

    let tray = initialize_tray_icon();

    start_tray_icon_update_loop(tray);

    gtk::main();
    Ok(())
}
