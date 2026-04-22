use std::collections::HashMap;

use crate::logic::intervention::{trigger_intervention, InterventionType};
use crate::logic::actions::close_tab::choose_tab_to_close;

pub fn run(kps: i16, bps: i16, wps: i16, idle: i16) -> InterventionType {
    let mut scores: HashMap<&str, i16> = HashMap::new();

    // Scoring logic (same idea, safer routing later)
    scores.insert("Break", idle * 2 + if kps < 2 { 1 } else { 0 });
    scores.insert("CloseTabs", wps * 3);
    scores.insert("DND", bps * 2 + wps);

    let best = scores
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;

    match best {
        "Break" => InterventionType::TakeBreak { duration_secs: 300 },

        "CloseTabs" => {
            if let Some((id, title)) = choose_tab_to_close() {
                InterventionType::CloseTab { id, title }
            } else {
                InterventionType::TakeBreak { duration_secs: 300 }
            }
        }

        "DND" => InterventionType::EnableDnd,

        _ => InterventionType::TakeBreak { duration_secs: 300 },
    }
}