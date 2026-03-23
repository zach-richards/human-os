use gtk::prelude::*;
use gtk::{Menu, MenuItem};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use image::{DynamicImage, Rgba};
use image::imageops::FilterType;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use gtk::glib;

// --- Resize an icon ---
fn resize_icon(img: &DynamicImage, size: u32) -> DynamicImage {
    img.resize_exact(size, size, FilterType::Lanczos3)
}

// --- Recolor an icon ---
fn recolor_icon(path: &str, color: (u8, u8, u8)) -> DynamicImage {
    let mut img = image::open(path).expect("Failed to load icon");

    for pixel in img.as_mut_rgba8().unwrap().pixels_mut() {
        let alpha = pixel[3];
        if alpha > 0 {
            *pixel = Rgba([color.0, color.1, color.2, alpha]);
        }
    }

    img
}

// --- Save icon to persistent path ---
fn save_icon(img: &DynamicImage) -> String {
    let path = "/tmp/tray_icon.png"; // persistent file
    let resized = resize_icon(img, 24); // resize to 24x24
    resized.save(path).unwrap();
    path.to_string()
}

// --- Create AppIndicator ---
fn create_indicator(icon_path: &str) -> AppIndicator {
    let mut indicator = AppIndicator::new("my-indicator", icon_path);
    indicator.set_status(AppIndicatorStatus::Active);
    indicator
}

// --- Update icon safely ---
fn update_icon(indicator: &mut AppIndicator, base_icon: &str, color: (u8, u8, u8)) {
    let img = recolor_icon(base_icon, color);
    let path = save_icon(&img);
    indicator.set_icon(&path);
}

fn main() {
    gtk::init().unwrap();

    let base_icon = "src/user11.png"; // your base icon path

    // --- Wrap AppIndicator for safe mutation ---
    let mut indicator = Rc::new(RefCell::new(create_indicator(base_icon)));

    // --- Menu setup ---
    let mut menu = Menu::new();
    let quit = MenuItem::with_label("Quit");
    quit.connect_activate(|_| gtk::main_quit());
    menu.append(&quit);
    menu.show_all();

    indicator.borrow_mut().set_menu(&mut menu);

    // --- Dynamic color cycle ---
    let colors = vec![(255, 0, 0), (0, 255, 0), (0, 0, 255)];
    let idx = Rc::new(RefCell::new(0));

    let indicator_clone = indicator.clone();
    let idx_clone = idx.clone();

    // --- Timer callback for updating icon ---
    glib::timeout_add_local(Duration::from_secs(2), move || {
        let mut ind = indicator_clone.borrow_mut();
        let mut i = idx_clone.borrow_mut();

        update_icon(&mut ind, base_icon, colors[*i]);
        *i = (*i + 1) % colors.len();

        glib::Continue(true) // keep timer running
    });

    // --- Start GTK main loop ---
    gtk::main();
}
