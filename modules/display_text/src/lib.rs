#![no_std]
//! This lib allows the kernel to display text on the screen. It provides macros like `prinŧ` and
//! `println`, and has some colors functionnality.
//!
//! ## processor_graphics
//!
//! This feature uses the integrated processor graphics to display text on the screen. Display
//! starts on the first line, and when the bottom is reached, it goes to the top of the screen. For
//! now, there is no way to get what is displayed on the screen, or the text previously displayed.
//!
//! You have to pass the `BootInfo` provided by the `bootloader_api` crate to init the display.
//! Then, just add `use display_text::DISPLAY_TEXT` and use macros to print text.

//---------------------------------------------------------------------------------
// feature processor graphics
#[cfg(feature = "processor_graphics")]
mod processor_graphics;
use bootloader_api::BootInfo;
#[cfg(feature = "processor_graphics")]
pub use processor_graphics::*;

//---------------------------------------------------------------------------------
// Common display_text interface. 
// This code is included in all cases, and it should be the only part of the crate used when

pub mod macros;

pub use core::fmt::Write;

#[derive(Debug)]
pub struct IndexOutOfRange;

pub trait DisplayText
where
    Self: core::fmt::Write + modules_common::InitModule,
{
    fn set_foreground_color(&mut self, color: (u8, u8, u8));
    fn set_background_color(&mut self, color: (u8, u8, u8));
    fn fill(&mut self, color: (u8, u8, u8));
    fn goto(&mut self, x: usize, y: usize) -> Result<(), IndexOutOfRange>;

    /// Move to the left on the screen, not on the text
    fn move_left(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
    /// Move to the right on the screen, not on the text
    fn move_right(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
    /// Move to the top on the screen, not on the text
    fn move_up(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
    /// Move to the bottom on the screen, not on the text
    fn move_down(&mut self, count: usize) -> Result<(), IndexOutOfRange>;
}
