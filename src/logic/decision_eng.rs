// decision_eng.rs

use crate::{logic::actions, notifications::notifications::Notification};

pub fn choose_focus_action(kps: i16, bps: i16, wps: i16, idle: i16) -> &'static str {
    let mut scores: std::collections::HashMap<&str, i16> = std::collections::HashMap::new();

    scores.insert("Break", idle * 2 + if kps < 2 {1} else {0});
    scores.insert("CloseTabs", wps * 3);
    scores.insert("DND", bps * 2 + wps);

    scores.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0
}

pub fn run(kps: i16, bps: i16, wps: i16, idle: i16) {

    let action = choose_focus_action(kps, bps, wps, idle);
    
    println!("Recommended Action: {}", action);

    if action == "Break" {
        //logic::actions::break_time::suggest_break();
    } else if action == "CloseTabs" {
        actions::close_tab::close_tab();
    } else if action == "DND" {
        //logic::actions::dnd_mode::enable_dnd();
    }
}