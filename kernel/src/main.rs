#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc};
use bootloader_api::{config::Mapping, entry_point, info::Optional, BootInfo, BootloaderConfig};
use kernel::{display::{self, text::DisplayTextManager}, interfaces::{CallInterface, InterfacesManager, Test}, memory::heap::init_heap};
use core::fmt::Write;

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = None;
    c.mappings.physical_memory = Some(Mapping::Dynamic);
    c
};

pub struct Main;

fn start(boot_info: &mut BootInfo) -> ! {
    let mut display_text = match &mut boot_info.framebuffer {
        Optional::Some(frame_buffer) => DisplayTextManager::new(frame_buffer),
        _ => panic!(),
    };

    init_heap(
        &boot_info.memory_regions,
        boot_info.physical_memory_offset.into_option().unwrap(),
    );

    writeln!(display_text, "Hello world!");
    let manager = InterfacesManager::new();
    let handle = manager.add_interface(Arc::new(Test));
    
    writeln!(display_text, "{:?}", handle.call(3));
    writeln!(display_text, "{:?}", manager.call(&handle, 3));

    loop {}
}

entry_point!(start, config = &CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // should trigger triple fault
    unsafe { *(0x0 as *mut u8) = 0 };
    loop {}
}

