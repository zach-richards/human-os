// intervention.rs

use notify_rust::{Notification, Timeout};
use std::thread;
use std::process::Command;

use crate::SYSTEM_INFO;
use crate::logic::actions::close_tab::close_tab;
use crate::logic::actions::dnd::{enable_dnd, disable_dnd};
use crate::logic::actions::take_break::send_break_notification;

// =========================
// TYPES
// =========================

pub enum InterventionType {
    CloseTab,
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
                enable_dnd();
            }
            InterventionType::TakeBreak { duration_secs } => {
                send_break_notification(duration_secs);
            }
        }
    });
}