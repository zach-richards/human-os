// tray_icon.rs

use gtk::prelude::*;
use gtk::{Menu, MenuItem};
use libayatana_appindicator::{AppIndicator, AppIndicatorStatus};
use image::Rgba;
// use image::imageops::FilterType;
use std::rc::Rc;
use std::cell::RefCell;
// use std::time::Duration;
// use gtk::glib;
use std::path::PathBuf;

pub struct TrayIcon {
    pub base_icon: PathBuf,
    pub indicator: Option<Rc<RefCell<AppIndicator>>>,
    pub colors: Vec<(u8, u8, u8)>,
    // pub idx: Rc<RefCell<usize>>,
}

impl TrayIcon {
    fn create_indicator(icon_path: &str) -> AppIndicator {
        let mut indicator = AppIndicator::new("my-indicator", icon_path);
        indicator.set_status(AppIndicatorStatus::Active);
        indicator
    }

    fn get_color_from_score(score: f32, colors: &[(u8, u8, u8)]) -> (u8, u8, u8) {
        let clamped = score.clamp(0.0, 1.0);

        let idx = if clamped <= 0.2 {
            0
        } else if clamped <= 0.4 {
            1
        } else if clamped <= 0.6 {
            2
        } else if clamped <= 0.8 {
            3
        } else {
            4
        };

        colors[idx]
    }

    pub fn new() -> Self {
        let base_icon = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("assets")
                        .join("icons")
                        .join("icon-neutral.png");

        let indicator = Self::create_indicator(&base_icon.to_str().unwrap());
        Self {
            base_icon,
            indicator: Some(Rc::new(RefCell::new(indicator))),
            colors: vec![(248, 113, 113), // fatigued
                         (236, 175, 117), // distracted
                         (217, 231, 122), // neutral
                         (52, 211, 153), // focused
                         (96, 165, 250)], // flow
            // idx: Rc::new(RefCell::new(0)),
        }
    }

    pub fn setup(&self, score: f32) {
        let mut menu = Menu::new();
        let quit = MenuItem::with_label("Quit");
        quit.connect_activate(|_| gtk::main_quit());
        menu.append(&quit);
        menu.show_all();

        if let Some(indicator_rc) = &self.indicator {
            let mut ind = indicator_rc.borrow_mut();
            let focus_fuel = format!("Focus Fuel: {}%", (score * 100.0).round());
            {
                let menu_ref: &mut Menu = &mut menu;
                ind.set_menu(menu_ref);
            }
            ind.set_title(&focus_fuel);
        }
    }

    pub fn run(&self, score: f32) {
        let indicator_clone = self.indicator.clone().unwrap();
        let base_icon = self.base_icon.clone();
        let self_clone = self.clone_for_closure();

        let mut ind = indicator_clone.borrow_mut();

        self_clone.update_icon(
            &mut *ind,
            &base_icon,
            Self::get_color_from_score(score, &self.colors),
            score,
        );
    }

    fn clone_for_closure(&self) -> TrayIconForClosure {
        TrayIconForClosure {
            // idx: self.idx.clone(),
        }
    }
}

pub struct TrayIconForClosure {
    // pub idx: Rc<RefCell<usize>>,
}

impl TrayIconForClosure {
    fn update_icon(&self, indicator: &mut AppIndicator, base_icon: &PathBuf, color: (u8, u8, u8), score: f32) {
        let mut img = image::open(base_icon).unwrap();
        for pixel in img.as_mut_rgba8().unwrap().pixels_mut() {
            let alpha = pixel[3];
            if alpha > 0 {
                *pixel = Rgba([color.0, color.1, color.2, alpha]);
            }
        }
        let path = format!("/tmp/tray_icon_{}.png", rand::random::<u32>());
        let focus_fuel = format!("Focus Fuel: {}%", (score * 100.0).round());
        img.save(&path).unwrap();
        indicator.set_icon(path.as_str());
        indicator.set_title(&focus_fuel);
    }
}
