// window.rs

use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

use crate::sys::system::SystemInfo;

fn get_window_name<C: Connection>(conn: &C, window: Window) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let net_wm_name = conn
        .intern_atom(false, b"_NET_WM_NAME")?
        .reply()?
        .atom;

    let utf8_string = conn
        .intern_atom(false, b"UTF8_STRING")?
        .reply()?
        .atom;

    let Ok(reply) = match conn
        .get_property(
            false,
            window,
            net_wm_name,
            utf8_string,
            0,
            u32::MAX,
        )?
    {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };

    if reply.value.is_empty() {
        return Ok(None);
    }

    let name = String::from_utf8(reply.value)?;
    Ok(Some(name))
}

pub fn handle_window_switch(sys_info: &mut SystemInfo) -> Result<(), Box<dyn Error>>{
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    let net_active = conn.intern_atom(false, b"_NET_ACTIVE_WINDOW")?.reply()?.atom;
    let reply = conn.get_property(false, root, net_active, AtomEnum::WINDOW, 0, 1)?.reply()?;

    if let Some(active_window) = reply.value32().and_then(|mut i| i.next()) {
        if let Ok(Some(name)) = get_window_name(&conn, active_window)? {
            println!("Active window {}", name);
        }
    }
}
