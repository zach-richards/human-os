// dnd.rs

use std::process::Command;
use std::thread;

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

    Ok(())
}