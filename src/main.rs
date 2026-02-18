// main.rs

mod sys;

use crate::sys::system::track_system;

fn main() {
    println!("Starting system tracker...");
    track_system().unwrap();
}
