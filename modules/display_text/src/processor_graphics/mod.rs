use bootloader_api::info::FrameBufferInfo;
use font::*;
use crate::DisplayText;

mod font;
mod init;
mod display_text;

pub use init::DISPLAY_TEXT;

#[derive(Debug, Clone, Copy)]
pub struct DisplayTextManager {
    frame_info: FrameBufferInfo,
    buffer: *mut u8,
    cursor: (usize, usize),
    foreground_color: (u8, u8, u8),
    background_color: (u8, u8, u8),
}

impl DisplayTextManager {
    fn set_pixel_uncheck(&mut self, position: (usize, usize), color_rgb: (u8, u8, u8)) {
        unsafe {
            *self.buffer.add(
                position.1 * self.frame_info.stride * self.frame_info.bytes_per_pixel
                    + position.0 * self.frame_info.bytes_per_pixel,
            ) = color_rgb.2;
            *self.buffer.add(
                position.1 * self.frame_info.stride * self.frame_info.bytes_per_pixel
                    + position.0 * self.frame_info.bytes_per_pixel
                    + 1,
            ) = color_rgb.1;
            *self.buffer.add(
                position.1 * self.frame_info.stride * self.frame_info.bytes_per_pixel
                    + position.0 * self.frame_info.bytes_per_pixel
                    + 2,
            ) = color_rgb.0;
        }
    }

    pub fn fill(&mut self, color_rgb: (u8, u8, u8)) {
        for x in 0..self.frame_info.width {
            for y in 0..self.frame_info.height {
                self.set_pixel_uncheck((x, y), color_rgb);
            }
        }
    }

    // TODO: cursor management position
    pub fn print(&mut self, text: &str) {
        for c in text.chars() {
            if c == '\n' {
                self.new_line();
            } else {
                self.write_char_uncheck(
                    c,
                    (self.cursor.0 * TOTAL_WIDTH, self.cursor.1 * TOTAL_EIGHT),
                    self.foreground_color,
                    self.background_color,
                );
                match self.move_right(1) {
                    Err(_) => self.new_line(),
                    _ => (),
                }
            }
        }
    }

    fn new_line(&mut self) {
        match self.goto(0, self.cursor.1 + 1) {
            Err(_) => self.goto(0, 0).unwrap(),
            _ => ()
        }
    }

    fn write_char_uncheck(
        &mut self,
        c: char,
        position: (usize, usize),
        foreground: (u8, u8, u8),
        background: (u8, u8, u8),
    ) {
        let pixels = font::font_char(c, background, foreground);

        for (y, line) in pixels.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                self.set_pixel_uncheck((position.0 + x, position.1 + y), *c);
            }
            for i in 0..SPACE_WIDTH {
                self.set_pixel_uncheck((position.0 + CHAR_WIDTH + i, position.1 + y), background);
            }
        }
        for i in 0..SPACE_HEIGHT {
            for x in 0..TOTAL_WIDTH {
                self.set_pixel_uncheck((position.0 + x, position.1 + CHAR_EIGHT + i), background);
            }
        }
    }
}
