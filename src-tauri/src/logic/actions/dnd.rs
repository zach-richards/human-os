// dnd.rs

// Sends message asking to turn on do not disturb. If accepted, turns back on in a little bit of time.

use std::{process::Command, thread};
use std::time::Duration;

use crate::notifications::notifications::Notification;

pub fn enable_dnd() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.notifications",
                "show-banners",
                "false",
            ])
            .output()
            .map_err(|e| format!("Failed to execute gsettings: {}", e))?;

        if !output.status.success() {
            return Err("Failed to enable DND".into());
        }
    }

    Notification::new(
        "Do Not Disturb Enabled",
        "Focus mode is now active",
        "OK",
        "Dismiss",
    )
    .send();

    // spawn timer thread
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(600)); // 10 minutes

        if let Err(e) = disable_dnd() {
            eprintln!("Failed to disable DND: {}", e);
        }
    });

    Ok(())
}

pub fn disable_dnd() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.notifications",
                "show-banners",
                "true",
            ])
            .output()
            .map_err(|e| format!("Failed to execute gsettings: {}", e))?;

        if !output.status.success() {
            return Err("Failed to disable DND".into());
        }
    }

    Notification::new(
        "Do Not Disturb Disabled",
        "Focus mode has ended",
        "OK",
        "Dismiss",
    )
    .send();

    Ok(())
}