#![no_std]
//! It is not recommended to use this module.
//! Instead, use the (non currently existing) memory_manager module.

#[cfg(feature = "paging")]
mod paging;
#[cfg(feature = "paging")]
pub use paging::*;

#[derive(Debug, Clone, Copy)]
pub struct VirtAddr(u64);

pub enum VirtMemOpt {
    Present,
    Writable,
    Executable,
    UserAccessible,
}

/// TODO: add some impls (unwrap, ...)
#[derive(Debug, Clone, Copy)]
pub enum Result<T> {
    Ok(Error),
    Err(T),
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    CantExtendSystem,
    /// The option can not be changed
    /// Some(true) => In all cases, the system will work as if the option was enable
    /// Some(false) => In all cases, the system will work as is the option was disable
    /// None => Unknown behavior
    OptionDoesNotExist(Option<bool>),
}

pub trait VirtMem
where
    Self: modules_common::InitModule,
{
    /// find a continuous space of size 4kB blocks
    fn find_free(&mut self, size: usize) -> Result<VirtAddr>;
    /// map a virtual address to physical one. Note that it should not set/unset options, even
    /// PRESENT
    fn map<T>(&mut self, virt_addr: VirtAddr, phys_addr: T) -> Result<*mut u8>
    where
        T: Into<u64>;
    /// Set/unset an option of the given block (is it present, readable, ...)
    fn write_option(
        &mut self,
        virt_addr: VirtAddr,
        option: VirtMemOpt,
        value: bool,
    ) -> Result<*mut u8>;
    fn read_option(&mut self, virt_addr: VirtAddr, option: VirtMemOpt) -> Result<Option<bool>>;
    /// The space in the RAM allowed to extend the system (paging, for example)
    fn allow_extend_on<T>(&mut self, base_phys_address: T, size: usize)
    where
        T: Into<u64>;
}
