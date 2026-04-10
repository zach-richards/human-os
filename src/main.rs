// main.rs

mod sys;
mod logic;
mod ui;
mod notifications;

use std::time::{ Instant, Duration };
use std::sync::{ Arc, Mutex };
use std::thread::{self, yield_now};

use rdev::listen;
use once_cell::sync::Lazy;
use gtk::glib;
use gtk::prelude::*;

use crate::ui::tray_icon::TrayIcon;
use crate::logic::cognitive_model;
use crate::sys::system;

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

fn start_system_input_update_loop() {
    // listener thread
    thread::spawn(|| {
        listen(system::handle_input_event).unwrap();
        yield_now();
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

fn start_decision_engine_loop() {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30));

            let (key_count, backspace_count, window_switch_count, idle_secs) = {
                let sys_info_clone = SYSTEM_INFO.lock().unwrap();
                (
                    sys_info_clone.key_count,
                    sys_info_clone.backspace_count,
                    sys_info_clone.window_switch_count,
                    sys_info_clone.last_activity
                        .map(|t| Instant::now().duration_since(t).as_secs() as i16)
                        .unwrap_or(0),
                )
            };

            logic::decision_eng::run(key_count, backspace_count, window_switch_count, idle_secs);
        }
    });
}

fn main() -> Result<(), rdev::ListenError> {
    initialize_system_time();

    start_system_input_update_loop();

    start_window_info_update_loop();

    start_cog_model_and_sys_info_update_loop();

    // Initialize GTK for tray icon
    gtk::init().unwrap();

    // Create and initialize GTK Application for notifications
    let app = gtk::Application::builder()
        .application_id("com.human-os.app")
        .build();

    // Register actions for notifications
    let close_tab_action = gtk::gio::SimpleAction::new("close-tab", None);
    close_tab_action.connect_activate(|_, _| {
        println!("Close tab action triggered");
    });
    app.add_action(&close_tab_action);

    let dismiss_action = gtk::gio::SimpleAction::new("dismiss", None);
    dismiss_action.connect_activate(|_, _| {
        println!("Dismiss action triggered");
    });
    app.add_action(&dismiss_action);

    // Set the application in the notifications module
    notifications::notifications::set_gtk_app(app.clone().upcast());

    let tray = initialize_tray_icon();

    start_tray_icon_update_loop(tray);

    start_decision_engine_loop();

    gtk::main();

    Ok(())
}