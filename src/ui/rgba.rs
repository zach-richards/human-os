// rgba.rs

pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<RGBA>,
}

impl Image {
    pub fn new(width: usize, height: usize, color: RGBA) -> Self {
        let pixels = vec![color; width * height];
        Self { width, height, pixels }
    }

    pub fn print_pixel(&self, x: usize, y: usize) {
        let pixel = self.pixels[y * self.width + x];
        println!("Pixel at ({}, {}): {:?}", x, y, pixel);
    }

        // Recolor all non-transparent pixels
    pub fn recolor_non_transparent(&mut self, new_color: RGBA) {
        for pixel in self.pixels.iter_mut() {
            if pixel.a > 0 {
                // Keep alpha the same, change RGB
                pixel.r = new_color.r;
                pixel.g = new_color.g;
                pixel.b = new_color.b;
            }
        }
    }
}
