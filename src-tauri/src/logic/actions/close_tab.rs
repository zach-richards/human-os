// close_tab.rs

// Detects distracting tabs and send notification to close it if needed.
// Distracting tabs meaning only YouTube at the moment

use std::process::Command;
use std::thread;

use crate::auxillary::state::SYSTEM_INFO;
use crate::notifications::notifications::Notification;

// Uses window ID to close window
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

// Searches for those with context "distraction" to then ask to close
pub fn choose_tab_to_close() -> Option<(String, String)> {
    let sys_info = SYSTEM_INFO.lock().unwrap_or_else(|e| e.into_inner());

    // Check for distracting windows and close them if they are distracting
    for window in sys_info.windows.iter() {
        if window.context == "distraction" {
            return Some((window.id.clone(), window.title.clone()));
        }
    }

    None
}

pub fn send_close_tab_notification(id: String, title: String) {
        let message = format!("Close \"{}\"?", title);

        thread::spawn(move || {
            let notification = Notification::new(
                "Focus Alert",
                Box::leak(message.into_boxed_str()),
                "Close",
                "Dismiss",
            );

            let accepted_action = notification.send();

            if accepted_action {
                close_window_id(&id).unwrap();
            }
        });
}