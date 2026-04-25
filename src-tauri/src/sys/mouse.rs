use std::time::Instant;
use crate::sys::system::SystemInfo;

pub fn handle_button_press(sys: &mut SystemInfo) {
    sys.mouse_clicks += 1;

    // click = HARD activity
    sys.last_mouse_activity = Some(Instant::now());
    sys.last_activity = Some(Instant::now());
}

pub fn handle_mouse_move(sys: &mut SystemInfo) {
    sys.last_mouse_move = Some(Instant::now());

    // ONLY promote to full activity if user was previously idle
    if sys.last_activity
        .map(|t| t.elapsed().as_secs() > 2)
        .unwrap_or(true)
    {
        sys.last_activity = Some(Instant::now());
    }

    // still track motion separately
    sys.last_mouse_activity = Some(Instant::now());
}

pub fn handle_wheel_scroll(sys: &mut SystemInfo) {
    sys.scroll_events += 1;

    sys.last_mouse_activity = Some(Instant::now());

    if sys.last_activity
        .map(|t| t.elapsed().as_secs() > 3)
        .unwrap_or(true)
    {
        sys.last_activity = Some(Instant::now());
    }
}