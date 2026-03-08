// image.rs

use image::{io::Reader as ImageReader, RgbaImage, Rgba};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<RGBA>,
}

impl Image {
    // Create a new blank image
    pub fn new(width: usize, height: usize, color: RGBA) -> Self {
        let pixels = vec![color; width * height];
        Self { width, height, pixels }
    }

    // Import from a file (PNG, JPEG, etc.)
    pub fn from_file<P: AsRef<Path>>(path: P) -> image::ImageResult<Self> {
        let img = ImageReader::open(path)?.decode()?; // decode image
        let img = img.to_rgba8(); // convert to RGBA8

    let (width, height) = img.dimensions();
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for px in img.pixels() {
            pixels.push(RGBA { r: px[0], g: px[1], b: px[2], a: px[3] });
        }

        Ok(Self {
            width: width as usize,
            height: height as usize,
            pixels,
        })
    }

    // Print a pixel
    pub fn print_pixel(&self, x: usize, y: usize) {
        let pixel = &self.pixels[y * self.width + x];
        println!("Pixel at ({}, {}): {:?}", x, y, pixel);
    }

    // Recolor all non-transparent pixels
    pub fn recolor_non_transparent(&mut self, new_color: RGBA) {
        for pixel in self.pixels.iter_mut() {
            if pixel.a != 0 {
                pixel.r = new_color.r;
                pixel.g = new_color.g;
                pixel.b = new_color.b;
            }
        }
    }
}

fn main() -> image::ImageResult<()> {
    // Load an image from disk
    let mut img = Image::from_file("example.png")?;
    img.print_pixel(0, 0);

    // Recolor non-transparent pixels to red
    img.recolor_non_transparent(RGBA { r: 255, g: 0, b: 0, a: 255 });
    img.print_pixel(0, 0);

    Ok(())
}
