// decision_eng.rs

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
}