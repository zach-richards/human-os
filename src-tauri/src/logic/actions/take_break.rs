// take_break.rs

// Sends notification asking user if they would like to take a break.
// If accepted, shows timer notification.

use std::{thread, time::Duration};

use crate::notifications::notifications;
use notify_rust;

pub fn send_break_notification(duration_secs: u64) {
    let notification = notifications::Notification::new(
        "Take a Break",
        "You've been working for a while",
        "Take Break",
        "Later",
    );

    let user_accept_action = notification.send();

    if user_accept_action {
        start_break_timer(duration_secs);
    }
}

fn start_break_timer(seconds: u64) {
    thread::spawn(move || {
        let mut remaining = seconds;

        // Create notification and keep the handle
        let _notif = notify_rust::Notification::new()
            .summary("Break in progress")
            .body(&format!("{} seconds remaining", remaining))
            .show()
            .unwrap();

        while remaining > 0 {
            thread::sleep(Duration::from_secs(2));

            if remaining > 60 {
                remaining -= 60;
            } else {
                remaining = remaining.saturating_sub(1);
            }

            let _notif = notify_rust::Notification::new()
                .summary("Break in progress")
                .body(&format!("{} seconds remaining", remaining))
                .show()
                .unwrap();
        }

        // end notification
        notify_rust::Notification::new()
            .summary("Break Over")
            .body("Time to get back to work")
            .show()
            .unwrap();
    });
}