#![no_std]

/// TODO: define what exactly is relevant from the memory manager and what has to be managed by the
/// task manager. Define the API.

#[cfg(feature = "paging4_recursive")]
mod level4_recursive;
#[cfg(feature = "paging4_recursive")]
pub use level4_recursive::*;

/// `size` is the number of 4kB blocks. It may vary in the future.
/// null_mut means an error
///
/// It's the module reponsability to free memory after use. Don't forget it!
pub trait MemoryManager {
    // for tasks that needs unspecific memory
    fn needs(&mut self, size: usize, options: SafeMemoryOptions) -> *mut u8;
    unsafe fn unsafe_needs(&mut self, size: usize, options: UnsafeMemoryOptions) -> *mut u8;

    // for tasks that needs specific memory
    fn map(&mut self, phys_addr: u64, size: usize, options: SafeMemoryOptions) -> *mut u8;
    unsafe fn unsafe_map(&mut self, phys_addr: u64, size: usize, options: UnsafeMemoryOptions) -> *mut u8;

    // mark the memory as free
    fn free(&mut self, ptr: *mut u8, size: usize);


    fn read_unsafe_options(&mut self, ptr: *mut u8) -> UnsafeMemoryOptions;
    fn read_safe_options(&mut self, ptr: *mut u8) -> SafeMemoryOptions;
}

/// These options are not supposed to cause security problems, bugs, or conduce to unsafe situations
pub struct SafeMemoryOptions {
    // TODO: add options...
}

/// These options can conduce to security problems, bugs, or unsafe situations
pub struct UnsafeMemoryOptions {
    // TODO: add options...
}
