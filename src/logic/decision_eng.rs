pub fn choose_focus_action(kps: f32, bps: f32, wps: f32, idle: f32) -> &'static str {
    let mut scores = std::collections::HashMap::new();

    scores.insert("Break", idle * 2.0 + if kps < 2.0 {1.0} else {0.0});
    scores.insert("CloseTabs", wps * 3.0);
    scores.insert("DND", bps * 2.0 + wps);

    scores.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0
}