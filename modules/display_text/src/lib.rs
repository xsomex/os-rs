#![no_std]

pub use display_text_interface::{DisplayText, ReturnMode};

pub use core::fmt::Write;

#[cfg(feature = "processor_graphics")]
use display_text__processor_graphics as display_text_impl;

#[cfg(any(feature = "processor_graphics"))]
pub use display_text_impl::{DisplayTextManager, init, DISPLAY_TEXT};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (DISPLAY_TEXT.lock().write_fmt(format_args!($($arg)*)).unwrap())
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => (display_text::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! set_foreground_color {
    ($r: expr, $v: expr, $b: expr) => {
        DISPLAY_TEXT.lock().set_foreground_color(($r, $v, $b))
    };
}

#[macro_export]
macro_rules! set_background_color {
    ($r: expr, $v: expr, $b: expr) => {
        DISPLAY_TEXT.lock().set_background_color(($r, $v, $b))
    };
}

#[macro_export]
macro_rules! set_color_default {
    () => {
        display_text::set_foreground_color!(255, 255, 255);
        display_text::set_background_color!(0, 0, 0);
    };
}

#[macro_export]
macro_rules! fill {
    ($r: expr, $v: expr, $b: expr) => {DISPLAY_TEXT.lock().fill(($r, $v, $b))};
}

#[macro_export]
macro_rules! move_cursor {
    (left $count:expr) => {DISPLAY_TEXT.lock().move_left(count, display_text::ReturnMode::None)};
    (right $count:expr) => {DISPLAY_TEXT.lock().move_right(count, display_text::ReturnMode::None)};
    (up $count:expr) => {DISPLAY_TEXT.lock().move_up(count, display_text::ReturnMode::None)};
    (down $count:expr) => {DISPLAY_TEXT.lock().move_down(count, display_text::ReturnMode::None)};
}
