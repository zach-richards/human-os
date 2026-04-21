use std::collections::HashMap;

use crate::intervention::{trigger_intervention, InterventionType};
use crate::intervention::find_distraction_tab;

pub fn choose_focus_action(kps: i16, bps: i16, wps: i16, idle: i16) -> InterventionType {
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
            if let Some((id, title)) = find_distraction_tab() {
                InterventionType::CloseTab { id, title }
            } else {
                InterventionType::TakeBreak { duration_secs: 300 }
            }
        }

        "DND" => InterventionType::EnableDnd,

        _ => InterventionType::TakeBreak { duration_secs: 300 },
    }
}