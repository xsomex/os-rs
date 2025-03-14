use crate::{DisplayTextManager, DisplayText, IndexOutOfRange};
use core::fmt;

use super::font::{TOTAL_EIGHT, TOTAL_WIDTH};

unsafe impl Sync for DisplayTextManager {}
unsafe impl Send for DisplayTextManager {}

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
        if x < self.frame_info.width/TOTAL_WIDTH && y < self.frame_info.height/TOTAL_EIGHT {
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
