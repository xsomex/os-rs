#![no_std]

//---------------------------------------------------------------------------------
// feature processor graphics
#[cfg(feature = "processor_graphics")]
mod processor_graphics;
#[cfg(feature = "processor_graphics")]
pub use processor_graphics::*;

//---------------------------------------------------------------------------------
// Common display_text interface. 
// This code is included in all cases, and it should be the only part of the crate used when
// display_text is needed. If a module wants to provide a display_text interface (a display modules
// for example, the display_text crate should be used with no feature.

pub mod macros;

pub use core::fmt::Write;

#[derive(Debug)]
pub struct IndexOutOfRange;

pub trait DisplayText
where
    Self: core::fmt::Write,
{
    fn set_foreground_color(&mut self, color: (u8, u8, u8));
    fn set_background_color(&mut self, color: (u8, u8, u8));
    fn fill(&mut self, color: (u8, u8, u8));
    fn goto(&mut self, x: usize, y: usize) -> Result<(), IndexOutOfRange>;

    fn move_left(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
    fn move_right(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
    fn move_up(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
    fn move_down(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
}
