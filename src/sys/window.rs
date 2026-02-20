// window.rs

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;

#[derive(Debug)]
pub enum WindowError {
    ConnectionError,
    NoActiveWindow,
    PropertyError,
    Utf8Error,
}

pub fn get_active_window() -> Result<String, WindowError> {
    // 1️⃣ Connect to X server
    let (conn, screen_num) =
        RustConnection::connect(None).map_err(|_| WindowError::ConnectionError)?;

    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    // 2️⃣ Get _NET_ACTIVE_WINDOW atom
    let net_active_atom = conn
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")
        .map_err(|_| WindowError::PropertyError)?
        .reply()
        .map_err(|_| WindowError::PropertyError)?
        .atom;

    // 3️⃣ Get active window property
    let active_window = conn
        .get_property(
            false,
            root,
            net_active_atom,
            AtomEnum::WINDOW,
            0,
            1,
        )
        .map_err(|_| WindowError::PropertyError)?
        .reply()
        .map_err(|_| WindowError::PropertyError)?;

    let window = active_window
        .value32()
        .and_then(|mut v| v.next())
        .ok_or(WindowError::NoActiveWindow)?;

    // 4️⃣ Get _NET_WM_NAME atom
    let net_wm_name_atom = conn
        .intern_atom(false, b"_NET_WM_NAME")
        .map_err(|_| WindowError::PropertyError)?
        .reply()
        .map_err(|_| WindowError::PropertyError)?
        .atom;

    let utf8_string_atom = conn
        .intern_atom(false, b"UTF8_STRING")
        .map_err(|_| WindowError::PropertyError)?
        .reply()
        .map_err(|_| WindowError::PropertyError)?
        .atom;

    // 5️⃣ Get window title
    let title_reply = conn
        .get_property(
            false,
            window,
            net_wm_name_atom,
            utf8_string_atom,
            0,
            u32::MAX,
        )
        .map_err(|_| WindowError::PropertyError)?
        .reply()
        .map_err(|_| WindowError::PropertyError)?;

    let title = String::from_utf8(title_reply.value)
        .map_err(|_| WindowError::Utf8Error)?;

    Ok(title)
}
