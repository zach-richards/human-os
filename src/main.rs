// main.rs

mod sys;

use std::error::Error;

use sys::window::track_window_switch;
use sys::system::track_system;

fn main() -> Result<(), Box<dyn Error>> {
    track_window_switch()?;
    track_system().unwrap();
    Ok(())
}

