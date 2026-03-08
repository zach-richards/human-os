// tray-icon.rs

use image::{DynamicImage, ImageFormat};

use crate::ui::rgba::RGBA;
use crate::ui::image::Image;

const FLOW_COLOR: RGBA = RGBA::new(96, 165, 250, 255);
const FOCUS_COLOR: RGBA = RGBA::new(52, 211, 153, 255);
const NEUTRAL_COLOR: RGBA = RGBA::new(217, 231, 122, 255);
const DISTRACTED_COLOR: RGBA = RGBA::new(253, 230, 138, 255);
const FATIGUE_COLOR: RGBA = RGBA::new(248, 113, 113, 255);

use tray_icon::{TrayIconBuilder, icon::Icon};
use image::{RgbaImage, Rgba};

// Suppose you have your Image struct with RGBA pixels
fn image_to_tray_icon(img: Image) -> Icon {
    // Convert your Image struct to RgbaImage
    let mut rgba_img = RgbaImage::new(img.width as u32, img.height as u32);

    for y in 0..img.height {
        for x in 0..img.width {
            let p = &img.pixels[y * img.width + x];
            rgba_img.put_pixel(x as u32, y as u32, Rgba([p.r, p.g, p.b, p.a]));
        }
    }

    // Convert RgbaImage to PNG bytes in memory
    let mut png_bytes: Vec<u8> = Vec::new();
    rgba_img.write_to(&mut png_bytes, image::ImageFormat::Png)
        .expect("Failed to write PNG bytes");

    Icon::from_png_data(&png_bytes).expect("Failed to create tray icon")
}

pub fn tray_icon() {
    // Load your image first (from file or created dynamically)
    let img = Image::from_file("icon.png").unwrap(); // your Image struct

    // Convert to tray-icon
    let icon = image_to_tray_icon(&img);

    // Build the system tray icon
    let tray_icon = TrayIconBuilder::new()
        .icon(icon)
        .tooltip("My Rust Tray Icon")
        .build()
        .expect("Failed to create tray icon");

    // Keep the app running to see the icon
    std::thread::park();
}
