// take_break.rs

use std::{thread, time::Duration};

use crate::notifications::notifications::Notification;

pub fn send_break_notification(duration_secs: u64) {
    let notification = Notification::new(
        "Take a Break",
        "You've been working for a while",
        "Take Break",
        "Later",
    );

    notification.send();

    // In your current architecture, action handling is inside notify-rust layer
    // So we just assume "break" decision is handled in notification engine
    start_break_timer(duration_secs);
}

fn start_break_timer(seconds: u64) {
    thread::spawn(move || {
        Notification::new(
            "Break Started",
            "Relax for a moment",
            "Ok",
            "Dismiss",
        )
        .send();

        thread::sleep(Duration::from_secs(seconds));

        Notification::new(
            "Break Over",
            "Back to work",
            "Ok",
            "Dismiss",
        )
        .send();
    });
}