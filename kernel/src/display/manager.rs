use core::any::Any;

use alloc::boxed::Box;
use bootloader_api::info::{FrameBuffer, PixelFormat};

use crate::objects::{Error, ObjectFunction};

use super::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct DisplayManager {
    buffer: usize,
    buffer_byte_len: usize,
    screen_size: (usize, usize), // in px
    buffer_size: (usize, usize), // in px
    bytes_per_px: usize,
    px_format: PixelFormat,
}

impl DisplayManager {
    pub fn init(frame_buffer: &mut FrameBuffer) -> Self {
        let info = &frame_buffer.info();
        let screen_size = (info.width, info.height);
        let buffer_size = (info.stride, info.height);
        let buffer_byte_len = info.byte_len;
        let bytes_per_px = info.bytes_per_pixel;
        let px_format = info.pixel_format;

        let buffer = frame_buffer.buffer_mut();
        buffer.fill(0);
        let buffer = buffer.as_ptr() as usize;

        DisplayManager {
            buffer,
            buffer_byte_len,
            screen_size,
            buffer_size,
            bytes_per_px,
            px_format,
        }
    }
}

pub fn set_pixel(object: &Box<dyn Any>, request: (usize, usize, Color)) -> Result<(), Error> {
    let display_manager = match object.downcast_ref::<DisplayManager>() {
        Some(v) => *v,
        None => return Err(Error::TypeError),
    };
    let (x, y, color) = request;

    if x > display_manager.screen_size.0 || y > display_manager.screen_size.1 {
        return Err(Error::Other);
    }

    let to_write = color.to(display_manager.px_format, display_manager.bytes_per_px);
    let start_index = y * display_manager.buffer_size.0 * display_manager.bytes_per_px + x * display_manager.bytes_per_px;

    for (i, byte) in to_write.into_iter().enumerate() {
        let buf = unsafe {
            core::slice::from_raw_parts_mut(display_manager.buffer as *mut u8, display_manager.buffer_byte_len)
        };
        buf[start_index + i] = byte
    }

    Ok(())
}

pub struct SetPixel;
impl ObjectFunction for SetPixel {
    type Input = (usize, usize, Color);
    type Output = ();
}
