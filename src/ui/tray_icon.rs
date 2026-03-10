// tray-icon.rs

const ICONS_LIST: [&str; 22] = [ "assets/icon0.png", "assets/icon1.png", "assets/icon2.png", "assets/icon3.png",
"assets/icon4.png", "assets/icon5.png", "assets/icon6.png", "assets/icon7.png", "assets/icon8.png",
"assets/icon9.png", "assets/icon10.png", "assets/icon11.png", "assets/icon12.png", "assets/icon13.png",
"assets/icon14.png", "assets/icon15.png", "assets/icon16.png", "assets/icon17.png", "assets/icon18.png",
"assets/icon19.png", "assets/icon20.png", "assets/icon21.png" ];

use ksni::{Tray, TrayMethods, MenuItem, Icon};
use ksni::menu::StandardItem;
use image::{GenericImageView, ImageReader};

use std::fs;

#[derive(Default)]
struct MyTray {
    icon: Vec<Icon>,
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
        self.icon.clone()
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

#[tokio::main]
pub async fn start() {
    let mut i: usize = 0;

    loop {
        let icon = MyTray::load_icon(ICONS_LIST[i]);
        let tray = MyTray { icon };

        // Start the tray
        let _handle = tray
            .spawn()
            .await
            .expect("failed to start tray");

        println!("Tray started with custom PNG icon!");

        // Keep running
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        i += 1;
    }
}
