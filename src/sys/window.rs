// window.rs

use std::error::Error;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;

pub fn track_window_switch() -> Result<(), Box<dyn Error>> {
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
                println!("Window switched!");
            }
        }
    }
}
