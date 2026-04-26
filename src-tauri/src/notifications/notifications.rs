// notification.rs

// a wrapper of notify_rust with a set template for my app's notifications to use.

use notify_rust::Notification as Notify;

pub struct Notification {
    pub label: String,
    pub description: String,
    pub option1: String,
    pub option2: String,
}

impl Notification {
    pub fn new(
        label: impl Into<String>,
        description: impl Into<String>,
        option1: impl Into<String>,
        option2: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            description: description.into(),
            option1: option1.into(),
            option2: option2.into(),
        }
    }

    pub fn send(&self) -> bool {
        let handle = Notify::new()
            .summary(&self.label)
            .body(&self.description)
            .action("action1", &self.option1)
            .action("action2", &self.option2)
            .show();

        match handle {
            Ok(notification_handle) => {
                let mut result = false;

                notification_handle.wait_for_action(|action| {
                    result = matches!(action, "action1");
                });

                result
            }
            Err(_) => false,
        }
    }
}