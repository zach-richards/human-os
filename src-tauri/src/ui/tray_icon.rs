use std::time::{Duration, Instant};

use tauri::{
    AppHandle,
    Manager,
    tray::TrayIconBuilder,
    menu::{Menu, MenuItem},
};
use tauri::image::Image;
use image::RgbaImage;

use crate::auxillary::get_color_from_score::get_color_from_score;

// ===============================
// STATE (NO GLOBAL MUTEX CHAOS)
// ===============================
pub struct TrayState {
    pub tray: tauri::tray::TrayIcon,
    pub focus_item: MenuItem<tauri::Wry>,
    pub base_icon: RgbaImage,
    pub last_update: std::sync::Mutex<Instant>,
}

// ===============================
// ICON GENERATION (IN MEMORY ONLY)
// ===============================
fn apply_color(base: &RgbaImage, score: f32) -> Image {
    let mut img = base.clone();
    let color = get_color_from_score(score);

    for px in img.pixels_mut() {
        let a = px[3];
        if a > 0 {
            px[0] = color.0;
            px[1] = color.1;
            px[2] = color.2;
        }
    }

    let (w, h) = img.dimensions();
    Image::new_owned(img.into_raw(), w, h)
}

// ===============================
// SETUP TRAY (CALLED ONCE)
// ===============================
pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let focus_item = MenuItem::new(app, "Focus Fuel: 50%", true, None::<&str>)?;
    let open_item = MenuItem::new(app, "Open App", true, None::<&str>)?;
    let quit_item = MenuItem::new(app, "Quit", true, None::<&str>)?;

    let menu = Menu::new(app)?;
    menu.append(&focus_item)?;
    menu.append(&open_item)?;
    menu.append(&quit_item)?;

    let tray = TrayIconBuilder::new()
        .tooltip("Human OS")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "Open App" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.set_always_on_top(true);
                        let _ = window.set_always_on_top(false);
                    }
                }
                "Quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    let base_icon = image::open("icons/32x32.png")
        .map_err(|e| tauri::Error::from(anyhow::anyhow!(e)))?
        .to_rgba8();

    app.manage(TrayState {
        tray,
        focus_item,
        base_icon,
        last_update: std::sync::Mutex::new(Instant::now()),
    });

    Ok(())
}

// ===============================
// FAST UPDATE PATH (HOT LOOP)
// ===============================
pub fn update_focus_fuel(app: &AppHandle, score: f32) -> tauri::Result<()> {
    let state = app.state::<TrayState>();

    let now = Instant::now();
    let mut last = state.last_update.lock().unwrap();
    *last = now; // FORCE UPDATE (no debounce)

    let icon = apply_color(&state.base_icon, score);

    state.tray.set_icon(Some(icon))?;

    state.focus_item.set_text(format!(
        "Focus Fuel: {}%",
        (score * 100.0).round() as i32
    ))?;

    println!("🔥 update_focus_fuel called: score = {}", score);

    Ok(())
}