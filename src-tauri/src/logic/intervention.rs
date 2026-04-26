// intervention.rs

// Takes the intervention type and handles the correct execution of the type.

use std::thread;

use crate::logic::actions::dnd::enable_dnd;
use crate::logic::actions::take_break::send_break_notification;
use crate::logic::actions::close_tab::send_close_tab_notification;

pub enum InterventionType {
    CloseTab { id: String, title: String },
    EnableDnd,
    TakeBreak { duration_secs: u64 },
}

pub fn trigger_intervention(intervention: InterventionType) {
    thread::spawn(move || {
        match intervention {
            InterventionType::CloseTab { id, title } => {
                send_close_tab_notification(id, title);
            }
            InterventionType::EnableDnd => {
                enable_dnd().unwrap();
            }
            InterventionType::TakeBreak { duration_secs } => {
                send_break_notification(duration_secs);
            }
        }
    });
}