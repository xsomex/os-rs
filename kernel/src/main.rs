#![no_main]
#![no_std]
#![allow(non_snake_case)]
#![feature(sync_unsafe_cell, abi_x86_interrupt)]

use kernel::*;
use common::*;
use bootloader_api::{BootloaderConfig, config::Mapping, BootInfo, entry_point};

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = Some(Mapping::Dynamic);
    c
};

fn start(info: &mut BootInfo) -> ! {
    display_text::init(info);
    fill!(0, 0, 0);
    println!("display_text successfully initialized");
    loop {}
}

entry_point!(start, config = &CONFIG);
