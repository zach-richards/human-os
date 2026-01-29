// sys_track.rs

/* This is a function that combines the checking of all system inputs into one listen function.
 * Data stored in each file dedicated to input to be assessed by another file to determine focus. */

use std::time::Duration;
use rdev::{Event, EventType, listen};

use crate::system::keyboard;

const THROTTLE: Duration = Duration::from_millis(50);

pub fn setup() {
    keyboard.setup();
    // mouse.setup();
    // window_track.setup();
}

pub fn track() {
    listen(move |event: Event| {
        match event.event_type {
            Some(EventType::KeyPress(_)) => {
                keyboard.active();
            }
            /* Some(EventType::MousePress(_)) {
                mouse.button_active();
            }
            Some(EventType::MouseMove {..} && last_mouse_move > THROTTLE) {
                mouse.movement_active();
            }
            Some(EventType::Wheel {..} && last_wheel_move > THROTTLE) {
                mouse.wheel_active();
            } */
            // window active
            // internet tab active
            // window/app name
            // tab title
            // activity/focus timing
            // Ex. Steam after Word ("Return focus to writing/studying?")
        }
    }).unwrap();
}
