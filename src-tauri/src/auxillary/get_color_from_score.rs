// get_color_from_score.rs

// gets focus color from focus fuel score

pub fn get_color_from_score(score: f32) -> (u8, u8, u8) {
    let s = score.clamp(0.0, 1.0);

    match s {
        s if s <= 0.2 => (248, 113, 113),
        s if s <= 0.4 => (236, 175, 117),
        s if s <= 0.6 => (217, 231, 122),
        s if s <= 0.8 => (52, 211, 153),
        _ => (96, 165, 250),
    }
}