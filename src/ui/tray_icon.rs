// tray_icon.rs

use gtk::prelude::*;
use gtk::{Menu, MenuItem};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use image::{DynamicImage, Rgba};
use image::imageops::FilterType;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use gtk::glib;
use std::borrow::BorrowMut;

pub struct TrayIcon {
    pub base_icon: String,
    pub indicator: Option<Rc<RefCell<AppIndicator>>>,
    pub colors: Vec<(u8, u8, u8)>,
    pub idx: Rc<RefCell<usize>>,
}

impl TrayIcon {
    // --- Resize an icon ---
    fn resize_icon(img: &DynamicImage, size: u32) -> DynamicImage {
        img.resize_exact(size, size, FilterType::Lanczos3)
    }

    // --- Recolor an icon ---
    fn recolor_icon(path: String, color: (u8, u8, u8)) -> DynamicImage {
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
    fn save_icon(&self, img: &DynamicImage) -> String {
        let path = "/tmp/tray_icon.png"; // persistent file
        let resized = Self::resize_icon(img, 24); // resize to 24x24
        resized.save(path).unwrap();
        path.to_string()
    }

    // --- Create AppIndicator ---
    fn create_indicator(icon_path: String) -> AppIndicator {
        let mut indicator = AppIndicator::new("my-indicator", icon_path.as_str());
        indicator.set_status(AppIndicatorStatus::Active);
        indicator
    }

    // --- Update icon safely ---
    fn update_icon(&self, indicator: &mut AppIndicator, base_icon: String, color: (u8, u8, u8)) {
        let img = Self::recolor_icon(base_icon, color);
        let path = self.save_icon(&img);
        indicator.set_icon(&path);
    }

    pub fn new(&self) -> Self {
        Self {
            base_icon: "assets/icons/icon-neutral.png".to_string(),
            indicator: None,
            colors: vec![(255, 0, 0), (0, 255, 0), (0, 0, 255)],
            idx: Rc::new(RefCell::new(0)),
        }
    }

    pub fn run(&self) {
        gtk::init().unwrap();

        self.new();

        // --- Wrap AppIndicator for safe mutation ---
        let mut indicator = Rc::new(RefCell::new(Self::create_indicator(self.base_icon)));

        // --- Menu setup ---
        let mut menu = Menu::new();
        let quit = MenuItem::with_label("Quit");
        quit.connect_activate(|_| gtk::main_quit());
        menu.append(&quit);
        menu.show_all();

        if let idx = &indicator {
            // borrow the inner AppIndicator mutably
            let mut idx_borrow = idx.borrow_mut();
            idx_borrow.set_menu(&menu); // pass &Menu
        }

        let indicator_clone = self.indicator.clone();
        let idx_clone = self.idx.clone();

        // --- Timer callback for updating icon ---
        glib::timeout_add_local(Duration::from_secs(2), move || {
            let mut ind = indicator_clone.borrow_mut();
            let mut i = idx_clone.borrow_mut();

            update_icon(&mut ind, self.base_icon, self.colors[*i]);
            *i = (*i + 1) % self.colors.len();

            glib::Continue(true) // keep timer running
        });

        gtk::main();
    }
}
