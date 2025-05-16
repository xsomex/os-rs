#![no_std]
#![no_main]

use alloc::{boxed::Box, vec::{self, Vec}};
use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping, entry_point, info::Optional};
use core::fmt::Write;
use kernel::{display::text::DisplayTextManager, memory::heap::init_heap};
extern crate alloc;

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = None;
    c.mappings.physical_memory = Some(Mapping::Dynamic);
    c
};

fn start(boot_info: &mut BootInfo) -> ! {
    let mut display_text = match &mut boot_info.framebuffer {
        Optional::Some(frame_buffer) => DisplayTextManager::new(frame_buffer),
        _ => panic!(),
    };

    display_text.write(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\n");
    writeln!(display_text, "Hello world!").unwrap();

    init_heap(
        &boot_info.memory_regions,
        boot_info.physical_memory_offset.into_option().unwrap(),
        &mut display_text,
    );

    // drop(vec) fails if there is vec.push(). If vec.push() is not called, drop(vec) works
    // perfectly

    let mut vec = Vec::with_capacity(4);
    vec.push(0u8);
    vec.push(0);
    vec.push(0);
    vec.push(0);
    vec.push(0);

    for i in &vec {
        write!(display_text, "{}", i);
    }
    drop(vec);

    let mut new_vec = alloc::vec![(), (), ()];
    new_vec.push(());
    for i in &new_vec {
        writeln!(display_text, "{:?}", i);
    }
    drop(new_vec);
    
    let b = Box::new("hello");
    writeln!(display_text, "{}", *b);
    drop(b);

    loop {}
}

entry_point!(start, config = &CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // should trigger triple fault
    // unsafe { *(0x0 as *mut u8) = 0 };
    loop {}
}
