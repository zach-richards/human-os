// close_tab.rs

use std::process::Command;
use std::thread;

use crate::SYSTEM_INFO;
use crate::notifications::notifications::Notification;

fn close_window_id(window_id: &str) -> Result<(), String> {
    if window_id.is_empty() {
        return Err("Window ID is empty".to_string());
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdotool")
            .arg("windowkill")
            .arg(window_id)
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("xdotool failed: {}", stderr));
        }
    }

    Ok(())
}

fn choose_tab_to_close() -> (Option<String>, Option<String>) {
    let sys_info = SYSTEM_INFO.lock().unwrap_or_else(|e| e.into_inner());

    for window in sys_info.windows.iter() {
        if window.context == "distraction" {
            return (Some(window.id.clone()), Some(window.title.clone()));
        }
    }

    (None, None)
}

pub fn close_tab(id: String, title: String) {
        let message = format!("Close \"{}\"?", title);

        // spawn so we don't block engine loop
        thread::spawn(move || {
            let notification = Notification::new(
                "Focus Alert",
                Box::leak(message.into_boxed_str()),
                "Close",
                "Dismiss",
            );

            // IMPORTANT:
            // action handling should live inside your Notification::send()
            // NOT here anymore
            notification.send();

            // If you still want auto-close behavior without user action fallback:
            // (optional safety behavior)
            println!("Notification sent for tab: {}", title);

            // DO NOT auto-close anymore unless explicitly triggered in notification engine
        });
}