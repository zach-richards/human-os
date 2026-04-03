// window.rs

use std::error::Error;
use std::sync::{ Arc, Mutex };
use std::thread;

use once_cell::sync::Lazy;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;

use crate::sys::system;

pub struct WindowInfo {
    pub id: u32,
    pub title: String,
    pub domain: Option<String>,
    pub timestamps: Vec<std::time::Instant>,
}

impl WindowInfo {
    pub fn new(id: u32, title: String, domain: Option<String>) -> Self {
        Self {
            id,
            title,
            domain,
            timestamps: Vec::new(),
        }
    }

    pub fn update_timestamp(&mut self) {
        self.timestamps.push(std::time::Instant::now());
    }

    pub fn time_spent(&self) -> std::time::Duration {
        if self.timestamps.len() < 2 {
            return std::time::Duration::new(0, 0);
        }
        let mut total = std::time::Duration::new(0, 0);
        for i in (1..self.timestamps.len()).step_by(2) {
            total += self.timestamps[i] - self.timestamps[i - 1];
        }
        total
    }
}

pub fn track_window_switches(sys_info: &Lazy<Arc<Mutex<system::SystemInfo>>>) -> Result<(), Box<dyn Error>> {
    // Connect to X server
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    // Get atom for _NET_ACTIVE_WINDOW
    let net_active_atom = conn
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")?
        .reply()?
        .atom;

    // Listen for property changes on root window
    conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::default()
            .event_mask(EventMask::PROPERTY_CHANGE),
    )?;

    conn.flush()?;

    loop {
        let event = conn.wait_for_event()?;

        if let Event::PropertyNotify(prop) = event {
            if prop.atom == net_active_atom {
                let mut sys_info_lock = sys_info.lock().unwrap();
                sys_info_lock.window_switch_count += 1;
                // println!("Window switched!");
            }
        }
        thread::yield_now();
    }
}
