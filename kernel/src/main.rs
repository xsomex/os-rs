#![no_std]
#![no_main]

// TODO:
// 1. Check Unused struct
// 2. Check GlobalAllocator::alloc()
// 3. Chack GlobalAllocator::dealloc()
// --
// The bug is probably in Unused or in alloc(), because even without drop(), if two Box are
// allocated, the second one panics.

use alloc::{
    alloc::alloc, boxed::Box, vec::{self, Vec}
};
use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping, entry_point, info::Optional};
use core::alloc::{GlobalAlloc, Layout};
use core::fmt::Write;
use kernel::{
    display::text::DisplayTextManager,
    memory::{global_allocator::GLOBAL_ALLOCATOR, heap::init_heap},
};
extern crate alloc;

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = None;
    c.mappings.physical_memory = Some(Mapping::Dynamic);
    c
};

macro_rules! debug_alloc {
    ($display_text: expr) => {
        let mut current_unused = GLOBAL_ALLOCATOR.first_unused.get();
        writeln!($display_text, "----------------------------");

        loop {
            writeln!(
                $display_text,
                "\nStart: {}\nSize: {}",
                current_unused.address, current_unused.size
            );
            match current_unused.next() {
                Ok(next) => {
                    current_unused = next;
                }
                _ => break,
            }
        }
    };
}

fn start(boot_info: &mut BootInfo) -> ! {
    let mut display_text = match &mut boot_info.framebuffer {
        Optional::Some(frame_buffer) => DisplayTextManager::new(frame_buffer),
        _ => panic!(),
    };

    display_text.write(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\n");

    init_heap(
        &boot_info.memory_regions,
        boot_info.physical_memory_offset.into_option().unwrap(),
        &mut display_text,
    );

    loop {}
}

entry_point!(start, config = &CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // should trigger triple fault
    unsafe { *(0x0 as *mut u8) = 0 };
    loop {}
}
