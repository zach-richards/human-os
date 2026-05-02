// take_break.rs

// Sends notification asking user if they would like to take a break.
// If accepted, shows a timer notification every 5 seconds until the break is over.

use std::{thread, time::Duration, process::Command};

use crate::notifications::notifications;

pub fn send_break_notification(duration_secs: u64) {
    let notification = notifications::Notification::new(
        "Take a Break",
        "You've been working for a while",
        "Take Break",
        "Later",
    );

    if notification.send() {
        start_break_timer(duration_secs);
    }
}

fn notify(summary: &str, body: &str) {
    let _ = Command::new("notify-send")
        .arg(summary)
        .arg(body)
        .spawn();
}

fn start_break_timer(seconds: u64) {
    thread::spawn(move || {
        let mut remaining = seconds;

        notify("Break in progress", &format_remaining(remaining));

        while remaining > 0 {
            let tick = remaining.min(5);
            thread::sleep(Duration::from_secs(tick));
            remaining = remaining.saturating_sub(tick);

            if remaining > 0 {
                notify("Break in progress", &format_remaining(remaining));
            }
        }

        notify("Break Over", "Time to get back to work");
    });
}

fn format_remaining(secs: u64) -> String {
    if secs >= 60 {
        let m = secs / 60;
        let s = secs % 60;
        format!("{}:{:02} remaining", m, s)
    } else {
        format!("{} seconds remaining", secs)
    }
}