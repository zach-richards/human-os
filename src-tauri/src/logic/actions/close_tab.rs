// close_tab.rs

use std::process::Command;

use crate::SYSTEM_INFO;
use crate::notifications::notifications::Notification;

fn close_window_id(window_id: &str) -> Result<(), String> {
    if window_id.is_empty() {
        return Err("Window ID is empty".to_string());
    }

    #[cfg(target_os = "linux")]
    {
        let close_output = Command::new("xdotool")
            .arg("windowkill")
            .arg(window_id)
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;

        if !close_output.status.success() {
            let stderr = String::from_utf8_lossy(&close_output.stderr);
            return Err(format!("xdotool failed: {}", stderr));
        }
    }

    Ok(())
}

fn choose_tab_to_close() -> (Option<String>, Option<String>) {
    println!("Attempting to lock SYSTEM_INFO...");

    let sys_info = SYSTEM_INFO.lock().unwrap_or_else(|e| {
        eprintln!("SYSTEM_INFO mutex poisoned, recovering: {}", e);
        e.into_inner()
    });

    println!("Successfully locked SYSTEM_INFO, checking {} windows", sys_info.windows.len());
    for window in sys_info.windows.iter() {
        println!("  Window: id={}, context={}", window.id, window.context);
        if window.context == "distraction" {
            println!("Found distraction window: {}", window.title);
            return (Some(window.id.clone()), Some(window.title.clone()));
        }
    }

    (None, None)
}

pub fn close_tab() {
    if let (Some(tab_id), Some(tab_title)) = choose_tab_to_close() {
        let message = format!("Focus fuel low! Close {}?", tab_title);
        let action_label = format!("Close {}", tab_title);
        let notification = Notification::new(
            "Focus Alert",
            Box::leak(message.into_boxed_str()),
            Box::leak(action_label.into_boxed_str()),
            "Dismiss"
        );
        notification.send();

        println!("Attempting to close window: id={}, title={}", tab_id, tab_title);
        match close_window_id(&tab_id) {
            Ok(_) => println!("Successfully closed {}", tab_title),
            Err(e) => eprintln!("Failed to close tab with id {}: {}", tab_id, e),
        }
    } else {
        println!("No distracting tabs found to close.");
    }
}