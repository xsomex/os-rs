use bootloader_api::info::PixelFormat;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 0xFF }
    }
    pub fn from_bgr(b: u8, g: u8, r: u8) -> Color {
        Color { r, g, b, a: 0xFF }
    }
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn from_bgra(b: u8, g: u8, r: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    pub fn to_bgr(self) -> (u8, u8, u8) {
        (self.b, self.g, self.r)
    }
    pub fn to_rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
    pub fn to_bgra(self) -> (u8, u8, u8, u8) {
        (self.b, self.g, self.r, self.a)
    }

    pub fn to(self, px_format: PixelFormat, _bytes_per_px: usize) -> Vec<u8> {
        let mut output = Vec::new();

        match px_format {
            PixelFormat::Rgb => {
                output.push(self.r);
                output.push(self.g);
                output.push(self.b);
                // for _ in 3..bytes_per_px {
                //     output.push(0xFF)
                // }
            }
            PixelFormat::Bgr => {
                output.push(self.b);
                output.push(self.g);
                output.push(self.r);
                // for _ in 3..bytes_per_px {
                //     output.push(0xFF)
                // }
            }
            PixelFormat::U8 => {
                // Why not
                output.push(self.r ^ self.g ^ self.b);
                // for _ in 1..bytes_per_px {
                //     output.push(0xFF)
                // }
            }
            _ => (),
        }

        output
    }
}
