// main.rs

mod sys;

use crate::sys::system::track_system;

fn main() {
    track_system().unwrap();
}

