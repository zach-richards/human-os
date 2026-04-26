// tray_icon.rs

// Creates a tray icon for the taskbar to show focus state and score

use tauri::{
    AppHandle,
    Manager,
    tray::TrayIconBuilder,
    menu::{Menu, MenuItem},
};

use image::{Rgba, RgbaImage};
use tauri::image::Image;

use crate::auxillary::get_color_from_score::get_color_from_score;

pub struct TrayManager;

impl TrayManager {
    pub fn new() -> Self {
        Self
    }

    // Generates the correct color for the state
    fn generate_colored_icon(&self, app: &AppHandle, score: f32) -> RgbaImage {
        let icon_path = app
            .path()
            .resolve(
                "icons/icon-neutral.png",
                tauri::path::BaseDirectory::Resource,
            )
            .expect("failed to resolve icon path");

        let img = image::open(icon_path).expect("icon load failed");
        let mut rgba = img.to_rgba8();

        let color = get_color_from_score(score);

        for px in rgba.pixels_mut() {
            let a = px[3];
            if a > 0 {
                *px = Rgba([color.0, color.1, color.2, a]);
            }
        }

        rgba
    }
}

// Sets up the tray icon with menu
pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let focus_item = MenuItem::new(app, "Focus Fuel 50%", true, None::<&str>)?;
    let quit_item = MenuItem::new(app, "Quit", true, None::<&str>)?;

    let menu = Menu::new(app)?;
    menu.append(&focus_item)?;
    menu.append(&quit_item)?;

    TrayIconBuilder::with_id("main")
        .tooltip("TEST TOOLTIP 123")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "Quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

// Updates the icon and the score on the menu of the icon
pub fn update_focus_fuel(app: &AppHandle, score: f32) -> tauri::Result<()> {
    let tray = app
        .tray_by_id("main")
        .expect("tray not found - ensure setup_tray() was called first");

    // generate icon in memory (NO temp files)
    let manager = TrayManager::new();
    let rgba = manager.generate_colored_icon(app, score);

    let (width, height) = rgba.dimensions();
    let bytes = rgba.into_raw();

    let icon = Image::new_owned(bytes, width, height);
    tray.set_icon(Some(icon))?;

    // rebuild menu (simple version)
    let menu = Menu::new(app)?;

    let focus_item = MenuItem::new(
        app,
        format!("Focus Fuel: {}%", (score * 100.0).round() as i32),
        true,
        None::<&str>,
    )?;

    let quit_item = MenuItem::new(app, "Quit", true, None::<&str>)?;

    menu.append(&focus_item)?;
    menu.append(&quit_item)?;

    tray.set_menu(Some(menu))?;

    Ok(())
}