#![no_std]
//! It is not recommended to use this module.
//! Instead, use the (non currently existing) memory_manager module.

pub struct VirtAddr(u64);

pub enum VirtMemOpt {}
pub enum Result<T> {
    Ok(Error),
    Err(T),
}

pub enum Error {
    CantExtendSystem,
}

pub trait VirtMem {
    /// find a continuous space of size 4kB blocks
    fn find_free(&mut self, size: usize) -> Result<VirtAddr>;
    /// map a virtual address to physical one. Note that it should not set/unset options, even
    /// PRESENT
    fn map<T>(&mut self, virt_addr: VirtAddr, phys_addr: T) -> Result<*mut u8>
    where
        T: Into<u64>;
    /// Set/unset an option of the given block (is it present, readable, ...)
    fn write_option(&mut self, virt_addr: VirtAddr, option: VirtMemOpt, value: bool) -> Result<*mut u8>;
    /// The space in the RAM allowed to extend the system (paging, for example)
    fn allow_extend_on<T>(&mut self, base_phys_address: T, size: usize)
    where
        T: Into<u64>;
}
