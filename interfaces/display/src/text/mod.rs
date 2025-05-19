//! TMP !!!!!!!!!
//! DO NOT USE !!
//! DEBUG PURPOSES ONLY !!!!!

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use bootloader_api::info::FrameBuffer;

use interfaces::{CallInterface, CallableObject, InterfaceHandle, InterfaceInputOutput};

use crate::font::GetChar;

use super::{color::Color, font::Font, manager::DisplayManager};

pub struct DisplayTextManager {
    display_manager: DisplayManager,
    cursor: (usize, usize),
    font: RefCell<InterfaceHandle<Box<dyn Font>>>,
}

impl DisplayTextManager {
    pub fn new(frame_buffer: &mut FrameBuffer, font: InterfaceHandle<Box<dyn Font>>) -> Self {
        let display_text = DisplayManager::init(frame_buffer);
        DisplayTextManager {
            display_manager: display_text,
            cursor: (0, 0),
            font: RefCell::new(font),
        }
    }

    pub fn write_char(&self, x: usize, y: usize, c: char) {
        let start = (x, y);
        for (y, line) in
            <InterfaceHandle<Box<dyn Font>> as CallInterface<Box<dyn Font>, GetChar>>::call(
                &InterfaceHandle::new(),
                &mut self.font.borrow_mut(),
                c,
            )
            .unwrap()
            .iter()
            .enumerate()
        {
            for (x, byte) in line.into_iter().enumerate() {
                if *byte {
                    let _ = self.display_manager.set_pixel(
                        start.0 + x,
                        start.1 + y,
                        Color::from_rgb(255, 255, 255),
                    );
                } else {
                    let _ = self.display_manager.set_pixel(
                        start.0 + x,
                        start.1 + y,
                        Color::from_rgb(0, 0, 0),
                    );
                }
            }
        }
    }

    pub fn write(&mut self, string: &str) {
        for c in string.chars() {
            if c == '\n' {
                self.cursor = (0, self.cursor.1 + 1);
            } else {
                self.write_char(
                    self.cursor.0 * 8,  // TMP HARDCODED
                    self.cursor.1 * 16, // TMP HARDCODED
                    c,
                );
                self.cursor = (self.cursor.0 + 1, self.cursor.1);
            }
        }
    }

    pub fn wrap(self) -> DisplayTextWrapper {
        DisplayTextWrapper(RefCell::new(self))
    }
}

use core::{
    cell::{Ref, RefCell},
    fmt,
};

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
    fn call(
        &self,
        inputs: <WriteStr as InterfaceInputOutput>::Input,
    ) -> <WriteStr as InterfaceInputOutput>::Output {
        self.write(inputs);
    }
}

pub struct DisplayTextHandle(pub InterfaceHandle<DisplayTextWrapper>);

impl fmt::Write for DisplayTextHandle {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        InterfaceHandle::new()
            .call(&mut self.0, s.to_string())
            .unwrap();
        Ok(())
    }
}
