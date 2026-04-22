// dnd.rs

use std::{process::Command, thread};
use std::time::Duration;

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

    Ok(())
}