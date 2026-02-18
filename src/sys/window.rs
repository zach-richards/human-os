// window.rs

use std::time::Instant;
use std::thread;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

use crate::sys::system::{SYSTEM_INFO, SystemInfo};

fn get_window_name<C: Connection>(conn: &C, window: Window) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let net_wm_name = conn.intern_atom(false, b"_NET_WM_NAME")?.reply()?.atom;
    let utf8_string = conn.intern_atom(false, b"UTF8_STRING")?.reply()?.atom;

    let reply = conn
        .get_property(false, window, net_wm_name, utf8_string, 0, u32::MAX)?
        .reply()?;

    if reply.value.is_empty() {
        return Ok(None);
    }

    Ok(Some(String::from_utf8(reply.value)?))
}

/// Update the focused window in SystemInfo and print it
fn handle_window_switch(sys_info: &mut SystemInfo, conn: &RustConnection, net_active_atom: u32, root: Window) {
    if let Ok(reply) = conn.get_property(false, root, net_active_atom, AtomEnum::WINDOW, 0, 1).and_then(|c| c.reply()) {
        if let Some(active_window) = reply.value32().and_then(|mut i| i.next()) {
            if let Ok(Some(name)) = get_window_name(conn, active_window) {
                // Only update if the window actually changed
                if sys_info.focused_window.as_deref() != Some(&name) {
                    sys_info.focused_window = Some(name.clone());
                    sys_info.last_window_change = Some(Instant::now());
                    println!("Active window: {}", name);
                }
            }
        }
    }
}

pub fn start_tracking_thread() {

    // Connect to X server
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    // Subscribe to PropertyChange events on the root window
    conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::new()
            .event_mask(EventMask::PROPERTY_CHANGE),
    )?;

    // Get the atom for _NET_ACTIVE_WINDOW
    let net_active_atom = conn.intern_atom(false, b"_NET_ACTIVE_WINDOW")?.reply()?.atom;

    // Initial fetch so we print the current window right away
    handle_window_switch(&mut sys_info, &conn, net_active_atom, root);

    println!("Listening for active window changes...");

    let sys_info = Arc::clone(&SYSTEM_INFO);
    thread::spawn(move || loop {
    // Event loop
        let event = conn.wait_for_event()?; // Blocks until an event occurs
        if let x11rb::protocol::Event::PropertyNotify(evt) = event {
            if evt.atom == net_active_atom {
                // Active window changed, update SystemInfo
                handle_window_switch(&mut sys_info, &conn, net_active_atom, root);
            }
        }
    })
}
