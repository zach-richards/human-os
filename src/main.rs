// main.rs

mod sys;

use std::sync::{Arc, Mutex};

use crate::sys::system::SystemInfo;

fn main() {
    let system_info_w = Arc::new(Mutex::new(SystemInfo::new())); // tracks system actions and shares
                                                                // it with...
    let system_info_r = system.clone();  // ...this one

    system_info_w.lock().unwrap().track();
}

