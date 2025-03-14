#![no_main]
#![no_std]
#![allow(non_snake_case)]
#![feature(sync_unsafe_cell, abi_x86_interrupt)]

use bootloader_api::{BootloaderConfig, config::Mapping, BootInfo, entry_point};
use modules_common::*;

use display_text::*;

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = Some(Mapping::Dynamic);
    c
};

fn start(info: &mut BootInfo) -> ! {
    DisplayTextManager::init(info);
    fill!(0, 0, 0);
    println!("display_text successfully initialized");
    loop {}
}

entry_point!(start, config = &CONFIG);


/// Needs display_text to be initialized
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:#?}", info);
    loop {}
}
