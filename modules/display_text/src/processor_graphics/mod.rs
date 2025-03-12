use crate::{DisplayText, IndexOutOfRange};
use bootloader_api::{
    BootInfo,
    info::{FrameBufferInfo, PixelFormat},
};
use core::{fmt, ptr::null_mut};
use font::{CHAR_EIGHT, CHAR_WIDTH, SPACE_HEIGHT, SPACE_WIDTH, TOTAL_EIGHT, TOTAL_WIDTH};
use spin::Mutex;

mod font;

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

    // TODO: cursor management position
    pub fn print(&mut self, text: &str) {
        for c in text.chars() {
            if c == '\n' {
                self.cursor.1 += 1;
                self.cursor.0 = 0;
            } else {
                self.write_char_uncheck(
                    c,
                    (self.cursor.0 * TOTAL_WIDTH, self.cursor.1 * TOTAL_EIGHT),
                    self.foreground_color,
                    self.background_color,
                );
                self.cursor.0 += 1;
            }
            if self.frame_info.width - self.cursor.0 * TOTAL_WIDTH < TOTAL_WIDTH {
                self.cursor.0 = 0;
                self.cursor.1 += 1;
            }
            if self.frame_info.height - self.cursor.1 * TOTAL_EIGHT < TOTAL_EIGHT {
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

    fn move_up(&mut self, count: usize) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0, self.cursor.1 - count)
    }
    fn move_down(&mut self, count: usize) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0, self.cursor.1 + count)
    }
    fn move_right(&mut self, count: usize) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0 + count, self.cursor.1)
    }
    fn move_left(&mut self, count: usize) -> Result<(), IndexOutOfRange> {
        self.goto(self.cursor.0 - count, self.cursor.1)
    }
}
