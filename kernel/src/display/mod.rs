pub mod color;
pub mod text;
pub mod font;

use bootloader_api::info::{FrameBuffer, PixelFormat};
use color::Color;
use font::{monospace::MONOSPACE, Char};

#[derive(Debug)]
pub struct DisplayManager<'a> {
    buffer: &'a mut [u8],
    buffer_byte_len: usize,
    screen_size: (usize, usize), // in px
    buffer_size: (usize, usize), // in px
    bytes_per_px: usize,
    px_format: PixelFormat,
}

impl<'a> DisplayManager<'a> {
    pub fn init(frame_buffer: &'a mut FrameBuffer) -> Self {
        let info = &frame_buffer.info();
        let screen_size = (info.width, info.height);
        let buffer_size = (info.stride, info.height);
        let buffer_byte_len = info.byte_len;
        let bytes_per_px = info.bytes_per_pixel;
        let px_format = info.pixel_format;

        let buffer = frame_buffer.buffer_mut();

        DisplayManager {
            buffer,
            buffer_byte_len,
            screen_size,
            buffer_size,
            bytes_per_px,
            px_format,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), ()> {
        if x > self.screen_size.0 || y > self.screen_size.1 {
            return Err(());
        }

        let to_write = color.to(self.px_format, self.bytes_per_px);
        let start_index = y * self.buffer_size.0 * self.bytes_per_px + x * self.bytes_per_px;

        for (i, byte) in to_write.into_iter().enumerate() {
            self.buffer[start_index + i] = byte
        }

        Ok(())
    }

    pub fn write_char(&mut self, x: usize, y: usize, c: Char<32, 32>) {
        let start = (x, y);
        for (y, line) in c.bytes().into_iter().enumerate() {
            for (x, byte) in line.into_iter().enumerate() {
                if byte {
                    let _ =
                        self.set_pixel(start.0 + x, start.1 + y, Color::from_rgb(255, 255, 255));
                } else {
                    let _ = self.set_pixel(start.0 + x, start.1 + y, Color::from_rgb(0, 0, 0));
                }
            }
        }
    }
}
