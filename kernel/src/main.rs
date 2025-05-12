#![no_std]
#![no_main]

use bootloader_api::{config::Mapping, entry_point, info::Optional, BootInfo, BootloaderConfig};
use kernel::display::{color::Color, text::DisplayTextManager, DisplayManager};

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = Some(Mapping::Dynamic);
    c
};

fn start(boot_info: &mut BootInfo) -> ! {
    let mut display_text = match &mut boot_info.framebuffer {
        Optional::Some(frame_buffer) => {
            DisplayTextManager::new(frame_buffer)
        },
        _ => panic!(),
    };

    display_text.write(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
 

    loop {}
}

entry_point!(start, config = &CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // should trigger triple fault
    unsafe { *(0x0 as *mut u8) = 0 };
    loop {}
}
