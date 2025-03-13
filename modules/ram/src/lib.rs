// low level implementation to control the ram

#![no_std]

#[cfg(feature = "paging4_recursive")]
mod level4_recursive;
#[cfg(feature = "paging4_recursive")]
pub use level4_recursive::*;

pub trait RamManager {
    fn find_free_virt(&mut self, size: usize) -> *mut u8; // ptr to virt mem
    fn find_free_phys(&mut self, size: usize) -> u64; // phys address
    unsafe fn map(&mut self, phys: u64, virt: *mut u8) -> *mut u8; // return virt. Options should
                                                                   // left unchanged
    unsafe fn set_options(&mut self, ptr: *mut u8, options: RamOptions);
    fn read_options(&mut self, ptr: *mut u8) -> RamOptions;
}

pub struct RamOptions {
    // TODO: add options...
}
