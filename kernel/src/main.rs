#![no_std]
#![no_main]

use alloc::{boxed::Box, sync::Arc};
use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping, entry_point, info::Optional};
use core::fmt::Write;
use kernel::{
    debug_global_allocator, display::{
        font::{monospace::monospace, Font, GetChar},
        manager::{set_pixel, DisplayManager, SetPixel},
        text::{write_string, DisplayTextManager, WriteString},
    }, min_memory::heap::init_heap, objects::{ArcObjectHandle, Object, ObjectsManager}
};

extern crate alloc;

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = None;
    c.mappings.physical_memory = Some(Mapping::Dynamic);
    c
};

fn start(boot_info: &mut BootInfo) -> ! {
    init_heap(
        &boot_info.memory_regions,
        boot_info.physical_memory_offset.into_option().unwrap(),
    );

    let display_manager = match &mut boot_info.framebuffer {
        Optional::Some(v) => DisplayManager::init(v),
        Optional::None => panic!(),
    };

    let display_manager = Object::new(Box::new(display_manager));
    display_manager.set_fn::<SetPixel>(set_pixel);

    let font = Object::new(Box::new(Font));
    font.set_fn::<GetChar>(monospace);

    let store = ObjectsManager::new();

    let display_manager_handle = store.add_object(display_manager);
    let font_handle = store.add_object(font);

    let display_text = DisplayTextManager::new(font_handle, display_manager_handle);
    let display_text = Object::new(Box::new(Arc::new(display_text)));
    display_text.set_fn::<WriteString>(write_string);

    let mut display_text_handle = ArcObjectHandle(store.add_object(display_text));

    // write!(display_text_handle, "Hello world!\nThe longest string I'he ever seen!!!\nAnd MORE AND MORE AND MORE Again!\nMore chars just becausethey are chars! And MORE MORE MORE MORE MORE MORE!!!!!!!!!").unwrap();
    
    debug_global_allocator!(display_text_handle);

    loop {}
}

entry_point!(start, config = &CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // should trigger triple fault
    // unsafe { *(0x0 as *mut u8) = 0 };
    loop {}
}
