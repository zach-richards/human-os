// tray_icon.rs

use std::path::PathBuf;
use tauri::{AppHandle, tray::TrayIconBuilder, menu::Menu};
use image::Rgba;
use tauri::image::Image;

pub struct TrayIcon {
    base_icon: PathBuf,
}

impl TrayIcon {
    pub fn new() -> Self {
        Self {
            base_icon: PathBuf::from("icons/icon-neutral.png"),
        }
    }

    fn get_color_from_score(score: f32) -> (u8, u8, u8) {
        let s = score.clamp(0.0, 1.0);

        match s {
            s if s <= 0.2 => (248, 113, 113),
            s if s <= 0.4 => (236, 175, 117),
            s if s <= 0.6 => (217, 231, 122),
            s if s <= 0.8 => (52, 211, 153),
            _ => (96, 165, 250),
        }
    }

    fn generate_colored_icon(&self, score: f32) -> String {
        let img = image::open(&self.base_icon).expect("icon load failed");
        let mut rgba = img.to_rgba8();

        let color = Self::get_color_from_score(score);

        for px in rgba.pixels_mut() {
            let a = px[3];
            if a > 0 {
                *px = Rgba([color.0, color.1, color.2, a]);
            }
        }

        let path = format!("/tmp/tray_icon_{}.png", rand::random::<u32>());
        rgba.save(&path).unwrap();
        path
    }
}

pub fn setup_tray(app: &AppHandle) {
    let menu = Menu::new(app).unwrap();

    let _tray = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)
        .unwrap();
}

pub fn update_focus_fuel(app: &AppHandle, score: f32) {
    let tray = app.tray_by_id("main").unwrap();

    let icon_manager = TrayIcon::new();
    let icon_path = icon_manager.generate_colored_icon(score);

    let img = image::open(&icon_path).expect("icon load failed");

    // force RGBA8
    let rgba = img.to_rgba8();

    let (width, height) = rgba.dimensions();

    // get raw pixel buffer
    let bytes = rgba.into_raw();

    let image = Image::new_owned(bytes, width, height);

    tray.set_icon(Some(image)).unwrap();
}