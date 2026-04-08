pub struct Notification {
    pub label: &str,
    pub description: &str,
    pub option1: &str,
    pub option2: &str,
}

impl Notification {
    pub fn new(label: &str, description: &str, option1: &str, option2: &str) -> Self {
        Self {
            label,
            description,
            option1,
            option2,
        }
    }

    pub fn send(&self) {
        // Placeholder for sending the notification
        println!("Notification: {}", self.label);
        println!("Description: {}", self.description);
        println!("Option 1: {}", self.option1);
        println!("Option 2: {}", self.option2);

        let app = Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_activate(|app| {
            // Register actions
            let take_break = gtk::gio::SimpleAction::new("take-break", None);
            take_break.connect_activate(|_, _| {
                println!("Taking a break!");
            });
            app.add_action(&take_break);

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
                notification.add_button("Take a Break", "app.take-break");
                notification.add_button("Dismiss", "app.dismiss");
                app_clone.send_notification(Some("focus-alert"), &notification);
            });

            let window = ApplicationWindow::builder()
                .application(app)
                .title("Notify Test")
                .default_width(200)
                .default_height(100)
                .child(&button)
                .build();

            window.show_all();
        });

        app.run();
    }
}