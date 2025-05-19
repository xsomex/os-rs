#![no_std]
#![no_main]

extern crate alloc;

use core::{alloc::Layout, fmt::{Debug, Write}};

use alloc::{
    boxed::Box, string::String, sync::Arc, vec::Vec, alloc::GlobalAlloc
};
use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping, entry_point, info::Optional};
use itf_display::{
    font::{self, Font},
    text::{DisplayTextHandle, DisplayTextManager},
};
use kernel::{debug_global_allocator, memory::{global_allocator::GLOBAL_ALLOCATOR, heap::init_heap}};

use interfaces::{AddInterface, InterfacesManager};

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = None;
    c.mappings.physical_memory = Some(Mapping::Dynamic);
    c
};

#[derive(Debug)]
pub struct Main;

fn start(boot_info: &mut BootInfo) -> ! {
    init_heap(
        &boot_info.memory_regions,
        boot_info.physical_memory_offset.into_option().unwrap(),
    );

    let manager = InterfacesManager::new();

    let font = <InterfacesManager as AddInterface<Box<dyn Font>>>::add_interface(
        &manager,
        Arc::new(Box::new(font::monospace::Monospace)),
    );

    let mut display_text = match &mut boot_info.framebuffer {
        Optional::Some(frame_buffer) => {
            DisplayTextHandle(manager.add_interface(Arc::new(DisplayTextManager::new(frame_buffer, font).wrap())))
        }
        _ => panic!(),
    };

    writeln!(display_text, "Hello world!");
    debug_global_allocator!(display_text);

    let mut v = Vec::new();
    for i in 0..100 {
        v.push(i);
    }

    debug_global_allocator!(display_text);
    drop(v);
    debug_global_allocator!(display_text);

    writeln!(display_text, "Hello world! This is a long......................... text");


    loop {}
}

entry_point!(start, config = &CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // should trigger triple fault
    // unsafe { *(0x0 as *mut u8) = 0 };
    loop {}
}
