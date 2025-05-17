#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::String, sync::Arc};
use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping, entry_point, info::Optional};
use kernel::{
    display::text::DisplayTextManager,
    interfaces::{AddInterface, CallInterface, InterfacesManager},
    memory::heap::init_heap,
};

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = None;
    c.mappings.physical_memory = Some(Mapping::Dynamic);
    c
};

pub struct Main;

fn start(boot_info: &mut BootInfo) -> ! {
    init_heap(
        &boot_info.memory_regions,
        boot_info.physical_memory_offset.into_option().unwrap(),
    );
    
    let manager = InterfacesManager::new();
    
    let mut display_text = match &mut boot_info.framebuffer {
        Optional::Some(frame_buffer) => manager.add_interface(Arc::new(DisplayTextManager::new(frame_buffer).wrap())),
        _ => panic!(),
    };

    manager.call(&mut display_text, String::from("And... it WORKS!\n"));

    loop {}
}

entry_point!(start, config = &CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // should trigger triple fault
    unsafe { *(0x0 as *mut u8) = 0 };
    loop {}
}
