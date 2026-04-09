// close_tab.rs

use crate::SYSTEM_INFO;
use std::process::Command;

fn close_window_id(window_id: &str) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let close_output = Command::new("xdotool")
            .arg("windowclose")
            .arg(window_id)
            .output()
            .map_err(|e| e.to_string())?;

        if !close_output.status.success() {
            return Err(format!("Failed to close window {}: {}", window_id, String::from_utf8_lossy(&close_output.stderr)));
        }
    }

    Ok(())
}

fn choose_tab_to_close() -> (Option<String>, Option<String>) {
    for window in SYSTEM_INFO.lock().unwrap().windows.iter() {
        if window.context == "distraction" {
            return (Some(window.id.clone()), Some(window.title.clone()));
        }
    }

    (None, None)
}

pub fn close_tab() {
    if let (Some(tab_id), Some(tab_title)) = choose_tab_to_close() {
        if let Err(e) = close_window_id(&tab_id) {
            eprintln!("Failed to close tab with id {}: {}", tab_id, e);
        } else {
            println!("Closed {}", tab_title);
        }
    } else {
        println!("No distracting tabs found to close.");
    }
}