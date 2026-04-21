// close_tab.rs

use std::process::Command;
use std::thread;

use notify_rust::{Notification as Notify, Timeout};

use crate::SYSTEM_INFO;

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

pub fn close_tab() {
    if let (Some(tab_id), Some(tab_title)) = choose_tab_to_close() {
        let message = format!("Focus low. Close \"{}\"?", tab_title);
        let action_label = format!("Close");

        // spawn so we don't block your engine loop
        thread::spawn(move || {
            let handle = Notify::new()
                .summary("Focus Alert")
                .body(&message)
                .action("close", &action_label)
                .action("dismiss", "Dismiss")
                .timeout(Timeout::Milliseconds(8000))
                .show();

            match handle {
                Ok(notification) => {
                    notification.wait_for_action(|action| {
                        match action {
                            "close" => {
                                println!("User confirmed closing: {}", tab_title);

                                match close_window_id(&tab_id) {
                                    Ok(_) => println!("Closed {}", tab_title),
                                    Err(e) => eprintln!("Failed to close {}: {}", tab_title, e),
                                }
                            }
                            "dismiss" => {
                                println!("User dismissed notification");
                            }
                            "__closed" => {
                                println!("Notification closed without action");
                            }
                            _ => {}
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Failed to send notification: {:?}", e);
                }
            }
        });
    } else {
        println!("No distracting tabs found.");
    }
}