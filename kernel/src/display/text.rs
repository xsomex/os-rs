use core::any::Any;
use core::cell::RefCell;

use alloc::{boxed::Box, string::String, sync::Arc};

use crate::objects::{Error, ObjectFunction, ObjectHandle};

use super::font::GetChar;

use super::color::Color;
use super::manager::SetPixel;

#[derive(Debug, Clone)]
pub struct DisplayTextManager {
    display_manager: Arc<ObjectHandle>,
    cursor: RefCell<(usize, usize)>,
    font: Arc<ObjectHandle>,
}

impl DisplayTextManager {
    pub fn new(font: Arc<ObjectHandle>, display_manager: Arc<ObjectHandle>) -> Self {
        DisplayTextManager {
            display_manager,
            cursor: RefCell::new((0, 0)),
            font,
        }
    }

    pub fn write_char(&self, x: usize, y: usize, c: char) {
        let start = (x, y);
        for (y, line) in self.font.call::<GetChar>(c).unwrap().iter().enumerate() {
            for (x, byte) in line.into_iter().enumerate() {
                if *byte {
                    let _ = self.display_manager.call::<SetPixel>((
                        start.0 + x,
                        start.1 + y,
                        Color::from_rgb(255, 255, 255),
                    ));
                } else {
                    let _ = self.display_manager.call::<SetPixel>((
                        start.0 + x,
                        start.1 + y,
                        Color::from_rgb(0, 0, 0),
                    ));
                }
            }
        }
    }

    pub fn write(&self, string: &str) {
        for c in string.chars() {
            if c == '\n' {
                let new = (0, self.cursor.borrow().1 + 1);
                *self.cursor.borrow_mut() = new;
            } else {
                self.write_char(
                    self.cursor.borrow().0 * 8,  // TMP HARDCODED
                    self.cursor.borrow().1 * 16, // TMP HARDCODED
                    c,
                );
                let new = (self.cursor.borrow().0 + 1, self.cursor.borrow().1);
                *self.cursor.borrow_mut() = new;
            }
        }
    }
}

pub struct WriteString;
impl ObjectFunction for WriteString {
    type Input = String;
    type Output = ();
}

pub fn write_string(object: &Box<dyn Any>, request: String) -> Result<(), Error> {
    let display_text = match object.downcast_ref::<Arc<DisplayTextManager>>() {
        Some(v) => v.clone(),
        None => return Err(Error::TypeError),
    };
    display_text.write(&request);
    Ok(())
}
