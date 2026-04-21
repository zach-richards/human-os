// take_break.rs

fn send_break_notification(duration_secs: u64) {
    let handle = Notification::new()
        .summary("Take a Break")
        .body("You've been working a while")
        .action("break", "Take break")
        .action("dismiss", "Later")
        .timeout(Timeout::Milliseconds(8000))
        .show();

    if let Ok(notification) = handle {
        notification.wait_for_action(|action| {
            if action == "break" {
                start_break_timer(duration_secs);
            }
        });
    }
}

fn start_break_timer(seconds: u64) {
    thread::spawn(move || {
        Notification::new()
            .summary("Break Started")
            .body("Relax")
            .show()
            .ok();

        std::thread::sleep(std::time::Duration::from_secs(seconds));

        Notification::new()
            .summary("Break Over")
            .body("Back to work")
            .show()
            .ok();
    });
}