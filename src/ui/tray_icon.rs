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
use rand::random;

pub struct TrayIcon {
    pub base_icon: String,
    pub indicator: Option<Rc<RefCell<AppIndicator>>>,
    pub colors: Vec<(u8, u8, u8)>,
    pub idx: Rc<RefCell<usize>>,
}

impl TrayIcon {
    fn resize_icon(img: &DynamicImage, size: u32) -> DynamicImage {
        img.resize_exact(size, size, FilterType::Lanczos3)
    }

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

    fn save_icon(&self, img: &DynamicImage) -> String {
        let path = "/tmp/tray_icon.png";
        let resized = Self::resize_icon(img, 24);
        resized.save(path).unwrap();
        path.to_string()
    }

    fn create_indicator(icon_path: &str) -> AppIndicator {
        let mut indicator = AppIndicator::new("my-indicator", icon_path);
        indicator.set_status(AppIndicatorStatus::Active);
        indicator
    }

    fn update_icon(&self, indicator: &mut AppIndicator, base_icon: &str, color: (u8, u8, u8)) {
        let img = Self::recolor_icon(base_icon, color);
        let path = self.save_icon(&img);
        indicator.set_icon(&path);
    }

    pub fn new() -> Self {
        let base_icon = "assets/icons/icon-neutral.png".to_string();
        let indicator = Self::create_indicator(&base_icon);
        Self {
            base_icon,
            indicator: Some(Rc::new(RefCell::new(indicator))),
            colors: vec![(96, 165, 250),
                         (52, 211, 153),
                         (217, 231, 122),
                         (236, 175, 117),
                         (248, 113, 113)],
            idx: Rc::new(RefCell::new(0)),
        }
    }

    pub fn run(&self) {
        gtk::init().unwrap();

        let mut menu = Menu::new();
        let quit = MenuItem::with_label("Quit");
        quit.connect_activate(|_| gtk::main_quit());
        menu.append(&quit);
        menu.show_all();

        if let Some(indicator_rc) = &self.indicator {
            let mut ind = indicator_rc.borrow_mut();
            ind.set_menu(&mut menu);
        }

        let indicator_clone = self.indicator.clone().unwrap();
        let idx_clone = self.idx.clone();
        let base_icon = self.base_icon.clone();
        let colors = self.colors.clone();
        let self_clone = self.clone_for_closure(); // helper for calling update_icon

        glib::timeout_add_local(Duration::from_secs(2), move || {
            let mut ind = indicator_clone.borrow_mut();
            let mut i = idx_clone.borrow_mut();

            self_clone.update_icon(&mut ind, &base_icon, colors[*i]);
            *i = (*i + 1) % colors.len();

            glib::Continue(true)
        });
    }

    fn clone_for_closure(&self) -> TrayIconForClosure {
        TrayIconForClosure {
            idx: self.idx.clone(),
        }
    }
}

pub struct TrayIconForClosure {
    pub idx: Rc<RefCell<usize>>,
}

impl TrayIconForClosure {
    fn update_icon(&self, indicator: &mut AppIndicator, base_icon: &str, color: (u8, u8, u8)) {
        let mut img = image::open(base_icon).unwrap();
        for pixel in img.as_mut_rgba8().unwrap().pixels_mut() {
            let alpha = pixel[3];
            if alpha > 0 {
                *pixel = Rgba([color.0, color.1, color.2, alpha]);
            }
        }
        let path = format!("/tmp/tray_icon_{}.png", rand::random::<u32>());
        img.save(&path).unwrap();
        indicator.set_icon(&path);
    }
}
