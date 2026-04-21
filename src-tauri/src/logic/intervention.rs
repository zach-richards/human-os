// intervention.rs

use notify_rust::{Notification, Timeout};
use std::thread;
use std::process::Command;

use crate::SYSTEM_INFO;

// =========================
// TYPES
// =========================

pub enum InterventionType {
    CloseTab { id: String, title: String },
    EnableDnd,
    TakeBreak { duration_secs: u64 },
}

// =========================
// PUBLIC ENTRY
// =========================

pub fn trigger_intervention(intervention: InterventionType) {
    thread::spawn(move || {
        match intervention {
            InterventionType::CloseTab { id, title } => {
                send_close_tab_notification(id, title);
            }
            InterventionType::EnableDnd => {
                send_dnd_notification();
            }
            InterventionType::TakeBreak { duration_secs } => {
                send_break_notification(duration_secs);
            }
        }
    });
}