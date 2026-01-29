// windows.rs

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use std::thread::sleep;
use std::time::Duration;

pub fn track_tab_switches() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    let mut last_window = 0;

    loop {
        // get active
        let reply = conn.get_property(
            false,
            root,
            conn.intern_atom(false, b"_NET_ACTIVE_WINDOW")?.reply()?.atom,
            AtomEnum::WINDOW,
            0,
            1,
        )?.reply()?;

        if let Some(window_id) = reply.value32().and_then(|mut v| v.next()) {
            if window_id != last_window {
                println!("Switched to window: {}", window_id);
                last_window = window_id;
            }
        }

        sleep(Duration::from_millis(200));
    }
}
