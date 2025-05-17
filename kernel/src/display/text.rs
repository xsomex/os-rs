//! TMP !!!!!!!!!
//! DO NOT USE !!
//! DEBUG PURPOSES ONLY !!!!!

use alloc::string::String;
use bootloader_api::info::FrameBuffer;

use crate::interfaces::{CallableObject, InterfaceInputOutput};

use super::{
    DisplayManager,
    font::{FontsList, monospace::MONOSPACE},
};

pub struct DisplayTextManager {
    display_text: DisplayManager,
    cursor: (usize, usize),
    font: FontsList,
}

impl<'a> DisplayTextManager {
    pub fn new(frame_buffer: &mut FrameBuffer) -> Self {
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

    pub fn wrap(self) -> DisplayTextWrapper {
        DisplayTextWrapper(RefCell::new(self))
    }
}

use core::{cell::RefCell, fmt};

impl fmt::Write for DisplayTextManager {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

pub struct DisplayTextWrapper(RefCell<DisplayTextManager>);

impl DisplayTextWrapper {
    pub fn write(&self, txt: String) {
        self.0.borrow_mut().write(&txt);
    }
}

pub struct WriteStr;
impl InterfaceInputOutput for WriteStr {
    type Input = String;
    type Output = ();
}

impl CallableObject<WriteStr> for DisplayTextWrapper {
    fn call(&self, inputs: <WriteStr as InterfaceInputOutput>::Input) -> <WriteStr as InterfaceInputOutput>::Output {
        self.write(inputs);
    }
}
