use notify_rust::{Notification as Notify, Timeout, Urgency, Hint};
use std::thread;

pub struct Notification {
    pub label: &'static str,
    pub description: &'static str,
    pub option1: &'static str,
    pub option2: &'static str,
}

impl Notification {
    pub fn new(
        label: &'static str,
        description: &'static str,
        option1: &'static str,
        option2: &'static str,
    ) -> Self {
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

        // spawn so we don't block your engine
        thread::spawn(move || {
            let handle = Notify::new()
                .summary(label)
                .body(description)
                .action("action1", option1)
                .action("action2", option2)
                .urgency(Urgency::Normal)
                .timeout(Timeout::Milliseconds(5000))
                .hint(Hint::Resident(true)) // keeps it until user interacts (if supported)
                .show();

            match handle {
                Ok(notification_handle) => {
                    // wait for user interaction
                    notification_handle.wait_for_action(|action| {
                        match action {
                            "action1" => {
                                println!("User clicked: {}", option1);
                                // 👉 hook your logic here
                                // e.g. close tab
                            }
                            "action2" => {
                                println!("User clicked: {}", option2);
                                // 👉 dismiss / ignore
                            }
                            "__closed" => {
                                println!("Notification closed");
                            }
                            _ => {}
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Failed to send notification: {:?}", e);
                }
            }
        });
    }
}