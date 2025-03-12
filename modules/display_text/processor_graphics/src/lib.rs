#![no_std]
#![allow(non_snake_case)]

use bootloader_api::{
    BootInfo,
    info::{FrameBufferInfo, PixelFormat},
};
use core::{fmt, ptr::null_mut};
use spin::Mutex;
use display_text_interface::{IndexOutOfRange, DisplayText};

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
        let B = background;
        let W = foreground;
        let pixels = match c {
            ' ' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
            ],
            'a' => [
                [B, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, B],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, W, W],
                [B, W, W, W, W, W, B, W],
            ],
            'b' => [
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
            ],
            'c' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, W],
            ],
            'd' => [
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, W, W, W, W, W, W],
            ],
            'e' => [
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, W],
            ],
            'f' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, W, W, W, W, W],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
            ],
            'g' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, W],
                [B, W, W, W, W, W, W, W],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
            ],
            'h' => [
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
            ],
            'i' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [W, W, W, W, W, W, W, W],
            ],
            'j' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, W, W, W, B, B],
                [B, B, B, B, B, W, B, B],
                [W, B, B, B, B, W, B, B],
                [W, B, B, B, B, W, B, B],
                [B, W, W, W, W, B, B, B],
            ],
            'k' => [
                [B, B, B, B, B, B, B, B],
                [W, B, B, B, B, W, W, B],
                [W, B, B, B, W, B, B, B],
                [W, B, B, W, B, B, B, B],
                [W, B, W, B, B, B, B, B],
                [W, W, B, B, B, B, B, B],
                [W, B, W, B, B, B, B, B],
                [W, B, B, W, W, B, B, B],
            ],
            'l' => [
                [B, B, B, B, B, B, B, B],
                [W, W, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, B, W, W, W, W, W],
            ],
            'm' => [
                [B, B, B, B, B, B, B, B],
                [B, W, W, B, W, W, W, B],
                [W, B, B, W, B, B, B, W],
                [W, B, B, W, B, B, B, W],
                [W, B, B, W, B, B, B, W],
                [W, B, B, W, B, B, B, W],
                [W, B, B, W, B, B, B, W],
                [W, B, B, W, B, B, B, W],
            ],
            'n' => [
                [B, B, B, B, B, B, B, B],
                [W, B, W, W, W, W, W, B],
                [W, W, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
            ],
            'o' => [
                [B, B, B, B, B, B, B, B],
                [B, B, W, W, W, W, B, B],
                [B, W, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, W, W, W, B, B],
            ],
            'p' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
            ],
            'q' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, W, W, W, W, W, W],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
            ],
            'r' => [
                [B, B, B, B, B, B, B, B],
                [W, B, W, W, W, W, W, W],
                [W, W, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
            ],
            's' => [
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, B],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
            ],
            't' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [W, W, W, W, W, W, W, W],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, B, W, W, W, W],
            ],
            'u' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, W, W, W, B, B],
            ],
            'v' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, B, B, W, B, B],
                [B, B, B, W, W, B, B, B],
                [B, B, B, W, B, B, B, B],
            ],
            'w' => [
                [B, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, B, W, W, B, W, B],
                [B, W, B, W, W, B, W, B],
                [B, W, B, W, B, B, W, B],
                [B, B, W, B, B, W, B, B],
                [B, B, W, B, B, W, B, B],
            ],
            'x' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, B, B, W, B, B],
                [B, B, B, W, W, B, B, B],
                [B, W, W, B, B, W, W, B],
                [W, B, B, B, B, B, B, W],
            ],
            'y' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, B, B, B, B, B, W],
                [B, B, W, B, B, B, W, B],
                [B, B, B, W, B, W, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, B, W, B, B, B, B],
                [W, W, W, B, B, B, B, B],
            ],
            'z' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
                [B, B, B, B, B, B, W, B],
                [B, B, B, B, W, W, B, B],
                [B, B, B, W, B, B, B, B],
                [B, W, W, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
            ],

            'A' => [
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
            ],
            'B' => [
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
            ],
            'C' => [
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, W],
            ],
            'D' => [
                [W, W, W, W, W, W, B, B],
                [W, B, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, W, B],
                [W, W, W, W, W, W, B, B],
            ],
            'E' => [
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
            ],
            'F' => [
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
            ],
            'G' => [
                [B, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, W, W, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, W, W, W, W, W, B],
            ],
            'H' => [
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
            ],
            'I' => [
                [W, W, W, W, W, W, W, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [W, W, W, W, W, W, W, B],
            ],
            'J' => [
                [B, B, B, W, W, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [W, B, B, B, B, W, B, B],
                [W, B, B, B, B, W, B, B],
                [B, W, W, W, W, B, B, B],
            ],
            'K' => [
                [W, B, B, B, W, W, B, B],
                [W, B, B, W, B, B, B, B],
                [W, B, W, B, B, B, B, B],
                [W, W, B, B, B, B, B, B],
                [W, B, W, B, B, B, B, B],
                [W, B, B, W, B, B, B, B],
                [W, B, B, B, W, B, B, B],
                [W, B, B, B, B, W, B, B],
            ],
            'L' => [
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
            ],
            'M' => [
                [W, B, B, B, B, B, B, W],
                [W, W, B, B, B, B, W, W],
                [W, B, W, B, B, W, B, W],
                [W, B, B, W, W, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
            ],
            'N' => [
                [W, B, B, B, B, B, B, W],
                [W, W, B, B, B, B, B, W],
                [W, B, W, B, B, B, B, W],
                [W, B, B, W, B, B, B, W],
                [W, B, B, B, W, B, B, W],
                [W, B, B, B, B, W, B, W],
                [W, B, B, B, B, B, W, W],
                [W, B, B, B, B, B, B, W],
            ],
            'O' => [
                [B, B, W, W, W, W, B, B],
                [B, W, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, W, W, W, B, B],
            ],
            'P' => [
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
            ],
            'Q' => [
                [B, B, W, W, W, W, B, B],
                [B, W, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, B, W],
                [B, B, W, W, W, W, W, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, B, W, W],
            ],
            'R' => [
                [W, W, W, W, W, W, W, B],
                [W, B, B, B, B, B, B, W],
                [W, W, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
                [W, B, W, B, B, B, B, B],
                [W, B, B, W, W, B, B, B],
                [W, B, B, B, B, W, B, B],
                [W, B, B, B, B, B, W, B],
            ],
            'S' => [
                [B, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, B],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, B],
            ],
            'T' => [
                [W, W, W, W, W, W, W, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
            ],
            'U' => [
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, W, W, W, B, B],
            ],
            'V' => [
                [W, B, B, B, B, B, W, B],
                [W, B, B, B, B, B, W, B],
                [W, B, B, B, B, B, W, B],
                [B, W, B, B, B, W, B, B],
                [B, W, B, B, B, W, B, B],
                [B, B, W, B, W, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
            ],
            'W' => [
                [W, B, B, B, B, B, W, B],
                [W, B, B, B, B, B, W, B],
                [W, B, B, W, B, B, W, B],
                [B, W, B, W, B, W, B, B],
                [B, W, B, W, B, W, B, B],
                [B, W, B, W, B, W, B, B],
                [B, B, W, B, W, B, B, B],
                [B, B, W, B, W, B, B, B],
            ],
            'X' => [
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, B, B, W, B, B],
                [B, B, B, W, W, B, B, B],
                [B, B, B, W, W, B, B, B],
                [B, B, W, B, B, W, B, B],
                [B, W, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
            ],
            'Y' => [
                [W, B, B, B, B, B, W, B],
                [B, W, B, B, B, W, B, B],
                [B, B, W, B, W, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
            ],
            'Z' => [
                [W, W, W, W, W, W, W, W],
                [B, B, B, B, B, B, W, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, W, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
            ],

            '0' => [
                [B, B, B, W, W, B, B, B],
                [B, B, W, B, B, W, B, B],
                [B, W, B, W, B, B, W, B],
                [B, W, B, W, B, B, W, B],
                [B, W, B, B, W, B, W, B],
                [B, W, B, B, W, B, W, B],
                [B, B, W, B, B, W, B, B],
                [B, B, B, W, W, B, B, B],
            ],
            '1' => [
                [B, W, W, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, W, W, W, W, W, B, B],
            ],
            '2' => [
                [B, B, W, W, W, W, B, B],
                [B, W, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, W, W, W, B],
                [B, B, W, W, B, B, B, B],
                [B, W, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
            ],
            '3' => [
                [B, W, W, W, W, W, B, B],
                [W, B, B, B, B, B, W, B],
                [B, B, B, B, B, B, B, W],
                [B, B, W, W, W, W, W, B],
                [B, B, B, B, B, B, B, W],
                [B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, W, B],
                [B, W, W, W, W, W, B, B],
            ],
            '4' => [
                [B, B, B, B, W, B, B, B],
                [B, B, B, W, W, B, B, B],
                [B, B, W, B, W, B, B, B],
                [B, W, B, B, W, B, B, B],
                [W, B, B, B, W, B, B, B],
                [W, W, W, W, W, W, W, W],
                [B, B, B, B, W, B, B, B],
                [B, B, B, B, W, B, B, B],
            ],
            '5' => [
                [W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, B, B],
                [B, B, B, B, B, B, W, B],
                [B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, W, B],
                [B, W, W, W, W, W, B, B],
            ],
            '6' => [
                [B, B, W, W, W, W, W, W],
                [B, W, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, B, B],
                [W, B, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, W, B],
                [B, B, W, W, W, W, B, B],
            ],
            '7' => [
                [W, W, W, W, W, W, W, W],
                [B, B, B, B, B, B, W, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, W, B, B, B, B, B, B],
                [W, B, B, B, B, B, B, B],
            ],
            '8' => [
                [B, B, W, W, W, W, B, B],
                [B, W, B, B, B, B, W, B],
                [B, W, B, B, B, B, W, B],
                [B, B, W, W, W, W, B, B],
                [B, W, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, W],
                [B, W, W, W, W, W, W, B],
            ],
            '9' => [
                [B, B, W, W, W, W, B, B],
                [B, W, B, B, B, B, W, B],
                [W, B, B, B, B, B, B, W],
                [B, W, B, B, B, B, B, W],
                [B, B, W, W, W, W, W, W],
                [B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, W, B],
                [B, W, W, W, W, W, B, B],
            ],

            '-' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
            ],
            '_' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [W, W, W, W, W, W, W, W],
            ],
            '"' => [
                [B, B, W, B, W, B, B, B],
                [B, B, W, B, W, B, B, B],
                [B, B, W, B, W, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
            ],
            '{' => [
                [B, B, B, B, W, W, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [W, W, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, B, W, W, B, B],
            ],
            '}' => [
                [B, B, W, W, B, B, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, W, W],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, W, W, B, B, B, B],
            ],
            '[' => [
                [B, B, W, W, W, W, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, W, W, W, B, B],
            ],
            ']' => [
                [B, B, W, W, W, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, W, W, W, W, B, B],
            ],
            '(' => [
                [B, B, B, B, W, W, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, W, B, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, B, W, W, B, B],
            ],
            ')' => [
                [B, B, W, W, B, B, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, B, W, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, W, W, B, B, B, B],
            ],
            ',' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, W, B, B, B, B, B],
            ],
            '=' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, W, W, W, W, W, W, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
            ],
            ':' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, W, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, W, B, B, B],
            ],
            '!' => [
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, W, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, W, B, B, B, B],
            ],
            '.' => [
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, B, B, B, B],
                [B, B, B, B, W, B, B, B],
            ],

            _ => [
                [W, W, W, W, W, W, W, W],
                [W, W, W, W, W, W, W, W],
                [W, W, W, W, W, W, W, W],
                [W, W, W, W, W, W, W, W],
                [W, W, W, W, W, W, W, W],
                [W, W, W, W, W, W, W, W],
                [W, W, W, W, W, W, W, W],
                [W, W, W, W, W, W, W, W],
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
            return Ok(())
        }
        Err(IndexOutOfRange)
    }
    
    fn move_up(&mut self, count: usize, return_mode: display_text_interface::ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0, self.cursor.1 - count)
    }
    fn move_down(&mut self, count: usize, return_mode: display_text_interface::ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0, self.cursor.1 + count)
    }
    fn move_right(&mut self, count: usize, return_mode: display_text_interface::ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0 + count, self.cursor.1)
    }
    fn move_left(&mut self, count: usize, return_mode: display_text_interface::ReturnMode) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0 - count, self.cursor.1)
    }
}

