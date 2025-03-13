use bootloader_api::{
    BootInfo,
    info::{FrameBufferInfo, PixelFormat},
};
use core::ptr::null_mut;
use spin::Mutex;
use crate::DisplayTextManager;

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

