use crate::{DisplayText, IndexOutOfRange, ReturnMode};
use bootloader_api::{
    BootInfo,
    info::{FrameBufferInfo, PixelFormat},
};
use core::{fmt, ptr::null_mut};
use spin::Mutex;

#[derive(Debug, Clone)]
pub struct DisplayTextManager {
    frame_info: FrameBufferInfo,
    buffer: *mut u8,
    cursor: (usize, usize),
    foreground_color: (u8, u8, u8),
    background_color: (u8, u8, u8),
}

pub static DISPLAY_TEXT: Mutex<DisplayTextManager> = Mutex::new(DisplayTextManager {
    frame_info: FrameBufferInfo {
        byte_len: 0,
        width: 0,
        height: 0,
        pixel_format: PixelFormat::U8,
        bytes_per_pixel: 0,
        stride: 0,
    },
    buffer: null_mut(),
    cursor: (0, 0),
    foreground_color: (255, 255, 255),
    background_color: (0, 0, 0),
});

unsafe impl Sync for DisplayTextManager {}
unsafe impl Send for DisplayTextManager {}

pub fn init(boot_info: &mut BootInfo) {
    match &mut boot_info.framebuffer {
        bootloader_api::info::Optional::None => panic!(),
        bootloader_api::info::Optional::Some(framebuffer) => {
            *DISPLAY_TEXT.lock() = DisplayTextManager {
                frame_info: framebuffer.info(),
                buffer: framebuffer.buffer_mut().as_ptr().cast_mut(),
                cursor: (0, 0),
                foreground_color: (255, 255, 255),
                background_color: (0, 0, 0),
            }
        }
    }
}

impl DisplayTextManager {
    pub fn set_pixel(
        &mut self,
        position: (usize, usize),
        color_rgb: (u8, u8, u8),
    ) -> Result<(), IndexOutOfRange> {
        if position.0 >= self.frame_info.width || position.1 >= self.frame_info.height {
            return Err(IndexOutOfRange);
        }
        self.set_pixel_uncheck(position, color_rgb);
        Ok(())
    }

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

    pub fn print(&mut self, text: &str) {
        for c in text.chars() {
            if c == '\n' {
                self.cursor.1 += 1;
                self.cursor.0 = 0;
            } else {
                self.write_char_uncheck(
                    c,
                    (self.cursor.0 * 10, self.cursor.1 * 10),
                    self.foreground_color,
                    self.background_color,
                );
                self.cursor.0 += 1;
            }
            if self.frame_info.width - self.cursor.0 * 10 < 10 {
                self.cursor.0 = 0;
                self.cursor.1 += 1;
            }
            if self.frame_info.height - self.cursor.1 * 10 < 10 {
                self.cursor.1 = 0;
            }
        }
    }

    fn write_char_uncheck(
        &mut self,
        c: char,
        position: (usize, usize),
        foreground: (u8, u8, u8),
        background: (u8, u8, u8),
    ) {
        let b = background;
        let w = foreground;
        let pixels = match c {
            ' ' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
            ],
            'a' => [
                [b, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, b],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, w, w],
                [b, w, w, w, w, w, b, w],
            ],
            'b' => [
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
            ],
            'c' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, w],
            ],
            'd' => [
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, w, w, w, w, w, w],
            ],
            'e' => [
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, w],
            ],
            'f' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, w, w, w, w, w],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
            ],
            'g' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, w],
                [b, w, w, w, w, w, w, w],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
            ],
            'h' => [
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
            ],
            'i' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [w, w, w, w, w, w, w, w],
            ],
            'j' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, w, w, w, b, b],
                [b, b, b, b, b, w, b, b],
                [w, b, b, b, b, w, b, b],
                [w, b, b, b, b, w, b, b],
                [b, w, w, w, w, b, b, b],
            ],
            'k' => [
                [b, b, b, b, b, b, b, b],
                [w, b, b, b, b, w, w, b],
                [w, b, b, b, w, b, b, b],
                [w, b, b, w, b, b, b, b],
                [w, b, w, b, b, b, b, b],
                [w, w, b, b, b, b, b, b],
                [w, b, w, b, b, b, b, b],
                [w, b, b, w, w, b, b, b],
            ],
            'l' => [
                [b, b, b, b, b, b, b, b],
                [w, w, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, b, w, w, w, w, w],
            ],
            'm' => [
                [b, b, b, b, b, b, b, b],
                [b, w, w, b, w, w, w, b],
                [w, b, b, w, b, b, b, w],
                [w, b, b, w, b, b, b, w],
                [w, b, b, w, b, b, b, w],
                [w, b, b, w, b, b, b, w],
                [w, b, b, w, b, b, b, w],
                [w, b, b, w, b, b, b, w],
            ],
            'n' => [
                [b, b, b, b, b, b, b, b],
                [w, b, w, w, w, w, w, b],
                [w, w, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
            ],
            'o' => [
                [b, b, b, b, b, b, b, b],
                [b, b, w, w, w, w, b, b],
                [b, w, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, w, w, w, b, b],
            ],
            'p' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
            ],
            'q' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, w, w, w, w, w, w],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
            ],
            'r' => [
                [b, b, b, b, b, b, b, b],
                [w, b, w, w, w, w, w, w],
                [w, w, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
            ],
            's' => [
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, b],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
            ],
            't' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [w, w, w, w, w, w, w, w],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, b, w, w, w, w],
            ],
            'u' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, w, w, w, b, b],
            ],
            'v' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, b, b, w, b, b],
                [b, b, b, w, w, b, b, b],
                [b, b, b, w, b, b, b, b],
            ],
            'w' => [
                [b, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, b, w, w, b, w, b],
                [b, w, b, w, w, b, w, b],
                [b, w, b, w, b, b, w, b],
                [b, b, w, b, b, w, b, b],
                [b, b, w, b, b, w, b, b],
            ],
            'x' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, b, b, w, b, b],
                [b, b, b, w, w, b, b, b],
                [b, w, w, b, b, w, w, b],
                [w, b, b, b, b, b, b, w],
            ],
            'y' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, b, b, b, b, b, w],
                [b, b, w, b, b, b, w, b],
                [b, b, b, w, b, w, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, b, w, b, b, b, b],
                [w, w, w, b, b, b, b, b],
            ],
            'z' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
                [b, b, b, b, b, b, w, b],
                [b, b, b, b, w, w, b, b],
                [b, b, b, w, b, b, b, b],
                [b, w, w, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
            ],

            'A' => [
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
            ],
            'B' => [
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
            ],
            'C' => [
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, w],
            ],
            'D' => [
                [w, w, w, w, w, w, b, b],
                [w, b, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, w, b],
                [w, w, w, w, w, w, b, b],
            ],
            'E' => [
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
            ],
            'F' => [
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
            ],
            'G' => [
                [b, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, w, w, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, w, w, w, w, w, b],
            ],
            'H' => [
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
            ],
            'I' => [
                [w, w, w, w, w, w, w, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [w, w, w, w, w, w, w, b],
            ],
            'J' => [
                [b, b, b, w, w, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [w, b, b, b, b, w, b, b],
                [w, b, b, b, b, w, b, b],
                [b, w, w, w, w, b, b, b],
            ],
            'K' => [
                [w, b, b, b, w, w, b, b],
                [w, b, b, w, b, b, b, b],
                [w, b, w, b, b, b, b, b],
                [w, w, b, b, b, b, b, b],
                [w, b, w, b, b, b, b, b],
                [w, b, b, w, b, b, b, b],
                [w, b, b, b, w, b, b, b],
                [w, b, b, b, b, w, b, b],
            ],
            'L' => [
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
            ],
            'M' => [
                [w, b, b, b, b, b, b, w],
                [w, w, b, b, b, b, w, w],
                [w, b, w, b, b, w, b, w],
                [w, b, b, w, w, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
            ],
            'N' => [
                [w, b, b, b, b, b, b, w],
                [w, w, b, b, b, b, b, w],
                [w, b, w, b, b, b, b, w],
                [w, b, b, w, b, b, b, w],
                [w, b, b, b, w, b, b, w],
                [w, b, b, b, b, w, b, w],
                [w, b, b, b, b, b, w, w],
                [w, b, b, b, b, b, b, w],
            ],
            'O' => [
                [b, b, w, w, w, w, b, b],
                [b, w, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, w, w, w, b, b],
            ],
            'P' => [
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
            ],
            'Q' => [
                [b, b, w, w, w, w, b, b],
                [b, w, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, b, w],
                [b, b, w, w, w, w, w, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, b, w, w],
            ],
            'R' => [
                [w, w, w, w, w, w, w, b],
                [w, b, b, b, b, b, b, w],
                [w, w, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
                [w, b, w, b, b, b, b, b],
                [w, b, b, w, w, b, b, b],
                [w, b, b, b, b, w, b, b],
                [w, b, b, b, b, b, w, b],
            ],
            'S' => [
                [b, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, b],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [w, w, w, w, w, w, w, b],
            ],
            'T' => [
                [w, w, w, w, w, w, w, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
            ],
            'U' => [
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, w, w, w, b, b],
            ],
            'V' => [
                [w, b, b, b, b, b, w, b],
                [w, b, b, b, b, b, w, b],
                [w, b, b, b, b, b, w, b],
                [b, w, b, b, b, w, b, b],
                [b, w, b, b, b, w, b, b],
                [b, b, w, b, w, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
            ],
            'W' => [
                [w, b, b, b, b, b, w, b],
                [w, b, b, b, b, b, w, b],
                [w, b, b, w, b, b, w, b],
                [b, w, b, w, b, w, b, b],
                [b, w, b, w, b, w, b, b],
                [b, w, b, w, b, w, b, b],
                [b, b, w, b, w, b, b, b],
                [b, b, w, b, w, b, b, b],
            ],
            'X' => [
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, b, b, w, b, b],
                [b, b, b, w, w, b, b, b],
                [b, b, b, w, w, b, b, b],
                [b, b, w, b, b, w, b, b],
                [b, w, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
            ],
            'Y' => [
                [w, b, b, b, b, b, w, b],
                [b, w, b, b, b, w, b, b],
                [b, b, w, b, w, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
            ],
            'Z' => [
                [w, w, w, w, w, w, w, w],
                [b, b, b, b, b, b, w, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, w, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
            ],

            '0' => [
                [b, b, b, w, w, b, b, b],
                [b, b, w, b, b, w, b, b],
                [b, w, b, w, b, b, w, b],
                [b, w, b, w, b, b, w, b],
                [b, w, b, b, w, b, w, b],
                [b, w, b, b, w, b, w, b],
                [b, b, w, b, b, w, b, b],
                [b, b, b, w, w, b, b, b],
            ],
            '1' => [
                [b, w, w, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, w, w, w, w, w, b, b],
            ],
            '2' => [
                [b, b, w, w, w, w, b, b],
                [b, w, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, w, w, w, b],
                [b, b, w, w, b, b, b, b],
                [b, w, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
            ],
            '3' => [
                [b, w, w, w, w, w, b, b],
                [w, b, b, b, b, b, w, b],
                [b, b, b, b, b, b, b, w],
                [b, b, w, w, w, w, w, b],
                [b, b, b, b, b, b, b, w],
                [b, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, w, b],
                [b, w, w, w, w, w, b, b],
            ],
            '4' => [
                [b, b, b, b, w, b, b, b],
                [b, b, b, w, w, b, b, b],
                [b, b, w, b, w, b, b, b],
                [b, w, b, b, w, b, b, b],
                [w, b, b, b, w, b, b, b],
                [w, w, w, w, w, w, w, w],
                [b, b, b, b, w, b, b, b],
                [b, b, b, b, w, b, b, b],
            ],
            '5' => [
                [w, w, w, w, w, w, w, w],
                [w, b, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, b, b],
                [b, b, b, b, b, b, w, b],
                [b, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, w, b],
                [b, w, w, w, w, w, b, b],
            ],
            '6' => [
                [b, b, w, w, w, w, w, w],
                [b, w, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, b, b],
                [w, b, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, w, b],
                [b, b, w, w, w, w, b, b],
            ],
            '7' => [
                [w, w, w, w, w, w, w, w],
                [b, b, b, b, b, b, w, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, w, b, b, b, b, b, b],
                [w, b, b, b, b, b, b, b],
            ],
            '8' => [
                [b, b, w, w, w, w, b, b],
                [b, w, b, b, b, b, w, b],
                [b, w, b, b, b, b, w, b],
                [b, b, w, w, w, w, b, b],
                [b, w, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, b, w],
                [b, w, w, w, w, w, w, b],
            ],
            '9' => [
                [b, b, w, w, w, w, b, b],
                [b, w, b, b, b, b, w, b],
                [w, b, b, b, b, b, b, w],
                [b, w, b, b, b, b, b, w],
                [b, b, w, w, w, w, w, w],
                [b, b, b, b, b, b, b, w],
                [w, b, b, b, b, b, w, b],
                [b, w, w, w, w, w, b, b],
            ],

            '-' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
            ],
            '_' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [w, w, w, w, w, w, w, w],
            ],
            '"' => [
                [b, b, w, b, w, b, b, b],
                [b, b, w, b, w, b, b, b],
                [b, b, w, b, w, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
            ],
            '{' => [
                [b, b, b, b, w, w, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [w, w, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, b, w, w, b, b],
            ],
            '}' => [
                [b, b, w, w, b, b, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, w, w],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, w, w, b, b, b, b],
            ],
            '[' => [
                [b, b, w, w, w, w, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, w, w, w, b, b],
            ],
            ']' => [
                [b, b, w, w, w, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, w, w, w, w, b, b],
            ],
            '(' => [
                [b, b, b, b, w, w, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, w, b, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, b, w, w, b, b],
            ],
            ')' => [
                [b, b, w, w, b, b, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, b, w, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, w, w, b, b, b, b],
            ],
            ',' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, w, b, b, b, b, b],
            ],
            '=' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, w, w, w, w, w, w, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
            ],
            ':' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, w, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, w, b, b, b],
            ],
            '!' => [
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, w, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, w, b, b, b, b],
            ],
            '.' => [
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, b, b, b, b],
                [b, b, b, b, w, b, b, b],
            ],

            _ => [
                [w, w, w, w, w, w, w, w],
                [w, w, w, w, w, w, w, w],
                [w, w, w, w, w, w, w, w],
                [w, w, w, w, w, w, w, w],
                [w, w, w, w, w, w, w, w],
                [w, w, w, w, w, w, w, w],
                [w, w, w, w, w, w, w, w],
                [w, w, w, w, w, w, w, w],
            ],
        };
        for y in 0..8 {
            for x in 0..8 {
                self.set_pixel_uncheck((position.0 + x, position.1 + y), pixels[y][x]);
            }
            self.set_pixel_uncheck((position.0 + 8, position.1 + y), background);
            self.set_pixel_uncheck((position.0 + 9, position.1 + y), background);
        }
        for i in 0..2 {
            for x in 0..10 {
                self.set_pixel_uncheck((position.0 + x, position.1 + 8 + i), background);
            }
        }
    }
}

impl fmt::Write for DisplayTextManager {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl DisplayText for DisplayTextManager {
    fn set_foreground_color(&mut self, color: (u8, u8, u8)) {
        self.foreground_color = color
    }
    fn set_background_color(&mut self, color: (u8, u8, u8)) {
        self.background_color = color
    }
    fn fill(&mut self, color: (u8, u8, u8)) {
        self.fill(color);
    }
    fn goto(&mut self, x: usize, y: usize) -> Result<(), IndexOutOfRange> {
        if x <= self.frame_info.width && y <= self.frame_info.height {
            self.cursor = (x, y);
            return Ok(());
        }
        Err(IndexOutOfRange)
    }

    fn move_up(&mut self, count: usize, _return_mode: ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0, self.cursor.1 - count)
    }
    fn move_down(&mut self, count: usize, _return_mode: ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0, self.cursor.1 + count)
    }
    fn move_right(&mut self, count: usize, _return_mode: ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0 + count, self.cursor.1)
    }
    fn move_left(&mut self, count: usize, _return_mode: ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0 - count, self.cursor.1)
    }
}
