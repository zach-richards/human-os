// notifications.rs

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use glib::clone;

const APP_ID: &str = "com.example.NotificationApp";

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
        let app = Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_activate(clone!(@weak app => move |_| {
            // Register actions
            let close_tab = gtk::gio::SimpleAction::new("close-tab", None);
            close_tab.connect_activate(|_, _| {
                println!("Closing tab!");
            });
            app.add_action(&close_tab);

            let dismiss = gtk::gio::SimpleAction::new("dismiss", None);
            dismiss.connect_activate(|_, _| {
                println!("Dismissed");
            });
            app.add_action(&dismiss);

            let button = Button::with_label("Send Notification");
            let app_clone = app.clone();
            button.connect_clicked(move |_| {
                let notification = gtk::gio::Notification::new("Focus Session");
                notification.set_body(Some("Your focus fuel is low!"));
                notification.add_button("Close Tab", "app.close-tab");
                notification.add_button("Dismiss", "app.dismiss");
                app_clone.send_notification(Some("focus-alert"), &notification);
            });

            let window = ApplicationWindow::builder()
                .application(&app)
                .title("Notify Test")
                .default_width(200)
                .default_height(100)
                .child(&button)
                .build();

            window.show_all();
        }));

        app.run();
    }
}