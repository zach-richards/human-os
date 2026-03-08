// rgba.rs

pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        Self { r, g, b, a }
    }
}
