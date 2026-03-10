// tray-icon.rs

use ksni::{Tray, TrayMethods, MenuItem, Icon};
use ksni::menu::StandardItem;
use image::{GenericImageView, ImageReader};

use std::fs;
use std::sync::Mutex;

use crate::logic::cognitive_model::CognitiveModel;

#[derive(Default)]
pub struct MyTray {
    pub icon: Vec<Vec<Icon>>,
    pub frame: Mutex<usize>,
}

impl MyTray {
    fn load_icon(path: &str) -> Vec<Icon> {
        // Load the PNG with the image crate
        let img = image::open(path).expect("Failed to open icon.png");

        // Convert into RGBA32
        let rgba = img.into_rgba8();
        let (width, height) = rgba.dimensions();
        assert_eq!(width, height, "Icon must be square!");

        // Convert RGBA to ARGB (swap bytes)
        let mut data = rgba.into_vec();
        for px in data.chunks_exact_mut(4) {
            // Pixel order RGBA → ARGB for ksni
            px.rotate_right(1);
        }

        vec![Icon {
            width: width as i32,
            height: height as i32,
            data,
        }]
    }
}

impl Tray for MyTray {
    fn id(&self) -> String {
        "rust_wayland_tray".into()
    }

    fn title(&self) -> String {
        "Rust Wayland Tray".into()
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        let i = *self.frame.lock().unwrap();
        self.icon[i].clone()
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            MenuItem::Standard(StandardItem {
                label: "Hello World".into(),
                activate: Box::new(|_| println!("Hello World clicked!")),
                ..Default::default()
            }),
            MenuItem::Standard(StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }),
        ]
    }
}

const FRAMES: &[&str] = &[
    "assets/icon0.png",
    "assets/icon1.png",
    "assets/icon2.png",
    "assets/icon3.png",
    "assets/icon4.png",
    "assets/icon5.png",
    "assets/icon6.png",
    "assets/icon7.png",
    "assets/icon8.png",
    "assets/icon9.png",
    "assets/icon10.png",
    "assets/icon11.png",
    "assets/icon12.png",
    "assets/icon13.png",
    "assets/icon14.png",
    "assets/icon15.png",
    "assets/icon16.png",
    "assets/icon17.png",
    "assets/icon18.png",
    "assets/icon19.png",
    "assets/icon20.png",
    "assets/icon21.png",
];

#[tokio::main]
pub async fn start(cog_model_clone: &mut CognitiveModel) {
    let icons: Vec<Vec<Icon>> = FRAMES
        .iter()
        .map(|p| {
            let icon = MyTray::load_icon(p);
            icon // must return icon here
        })
        .collect();

    let icons: Vec<Vec<Icon>> = FRAMES
        .iter()
        .map(|p| MyTray::load_icon(p))
        .collect();

    let tray = MyTray {
        icon: icons,
        frame: Mutex::new(0),
    };

    let handle = tray.spawn().await.expect("failed to start tray");
    
    let i = 0;

    loop {
        handle.update(|tray| {
            // update the frame index inside MyTray
            let mut frame = tray.frame.lock().unwrap();
            *frame = (*frame + 1) % tray.icon.len();

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        });
    }
}
