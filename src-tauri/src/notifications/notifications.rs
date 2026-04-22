use notify_rust::Notification as Notify;

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

    pub fn send(&self) -> bool {
        let label = self.label;
        let description = self.description;
        let option1 = self.option1;
        let option2 = self.option2;

        let handle = Notify::new()
            .summary(label)
            .body(description)
            .action("action1", option1)
            .action("action2", option2)
            .show();

        if let Ok(notification_handle) = handle {
            let mut result = false;

            notification_handle.wait_for_action(|action| {
                match action {
                    "action1" => result = true,
                    _ => result = false,
                }
            });

            return result;
        }

        false
    }
}