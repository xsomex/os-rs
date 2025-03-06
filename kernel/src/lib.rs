#![no_std]
#![feature(sync_unsafe_cell, abi_x86_interrupt)]
#![allow(non_snake_case)]

pub use display_text::*;

/// Needs display_text to be initialized
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:#?}", info);
    loop {}
}
