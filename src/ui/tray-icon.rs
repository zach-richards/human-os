use ksni::{Tray, TrayMethods, MenuItem, Icon};
use ksni::menu::StandardItem;
use image::GenericImageView;

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
async fn main() {
    // Load your PNG icon
    let icons = MyTray::load_icon("white-user-circle.png");
    let tray = MyTray { icon: icons };

    // Start the tray
    let _handle = tray
        .spawn()
        .await
        .expect("failed to start tray");

    println!("Tray started with custom PNG icon!");

    // Keep running
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
