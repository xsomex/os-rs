//! TMP !!!!!!!!!
//! DO NOT USE !!
//! DEBUG PURPOSES ONLY !!!!!

use bootloader_api::info::FrameBuffer;

use super::{
    font::{monospace::MONOSPACE, Font, FontsList}, DisplayManager
};

pub struct DisplayTextManager<'a> {
    display_text: DisplayManager<'a>,
    cursor: (usize, usize),
    font: FontsList,
}

impl<'a> DisplayTextManager<'a> {
    pub fn new(frame_buffer: &'a mut FrameBuffer) -> Self {
        let display_text = DisplayManager::init(frame_buffer);
        DisplayTextManager {
            display_text,
            cursor: (0, 0),
            font: FontsList::Monospace,
        }
    }
    pub fn write(&mut self, string: &str) {
        let font = match self.font {
            FontsList::Monospace => &MONOSPACE,
        };
        for c in string.chars() {
            if c == '\n' {
                self.cursor = (0, self.cursor.1 + 1);
            } else {
                self.display_text.write_char(
                    self.cursor.0 * font.width,
                    self.cursor.1 * font.height,
                    match font.char(c) {
                        Some(font_c) => font_c,
                        None => font.char(' ').unwrap(),
                    },
                );
                self.cursor = (self.cursor.0 + 1, self.cursor.1);
            }
        }
    }
}
