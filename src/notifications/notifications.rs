// notifications.rs

use gtk::{glib, prelude::*};
use gtk::Application;
use std::sync::OnceLock;

static GTK_APP: OnceLock<Application> = OnceLock::new();

pub fn set_gtk_app(app: Application) {
    let _ = GTK_APP.set(app);
}

pub struct Notification {
    pub label: &'static str,
    pub description: &'static str,
    pub option1: &'static str,
    pub option2: &'static str,
}

impl Notification {
    pub fn new(label: &'static str, description: &'static str, option1: &'static str, option2: &'static str) -> Self {
        Self {
            label,
            description,
            option1,
            option2,
        }
    }

    pub fn send(&self) {
        let label = self.label;
        let description = self.description;
        let option1 = self.option1;
        let option2 = self.option2;

        // Dispatch the notification send on the main thread
        glib::idle_add_local_once(move || {
            if let Some(app) = GTK_APP.get() {
                let notification = gtk::gio::Notification::new(label);
                notification.set_body(Some(description));
                notification.add_button(option1, "app.close-tab");
                notification.add_button(option2, "app.dismiss");
                app.send_notification(Some("focus-alert"), &notification);
            } else {
                eprintln!("GTK Application not initialized for sending notifications");
            }
        });
    }
}