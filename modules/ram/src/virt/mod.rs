#[cfg(feature = "virt_paging")]
mod paging;
use core::ptr::null_mut;

#[cfg(feature = "virt_paging")]
pub use paging::*;

//-------------------------------------------------------------

use super::phys::PhysAddr;

/// Represents a virtual address. Does not provide any functions, only usefull to make the
/// difference between physical and virtual addresses in function args or returns.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VirtAddr(u64);

/// Represent the different options that are available for a page. Options must be available in the
/// most cases, if it's a very specific option, add it with `#[cfg(feature = "feature")]`. Note
/// that if a module uses a feature specific option, it will not work with a different variant of
/// the crate. You should'nt do that.
#[derive(Debug, Clone, Copy)]
pub enum VirtMemOptions {
    /// bit index
    /// Note that implementation must warranty that the bit will not overwrite the address or an
    /// other option.
    Custom(usize),

    /// The page is present on memory or not.
    Present,

    /// The content of the page is executable or not.
    Executable,
    // TODO: add others
}

#[derive(Debug, Clone, Copy)]
pub enum VirtMemError {
    /// Returned when using `VirtMemOptions::Custom(x)`, if x is too much big. Hold the max value
    /// of the `Custom` option.
    CustomOptionOutOfRange(usize),

    /// Returned when an option does not exist on the used implementation. Holds `Some(true)` when
    /// the option is always enable, `Some(false)` if it is always disabled. `None` means that the
    /// default value for the option is unknown, or that you should try change it by another way.
    OptionDoesNotExist(Option<bool>),
}

pub trait VirtMemManager {
    /// Can use `PhysMemManager::find_free` and `VirtMemManager::map`
    fn find_free(&mut self, size: usize) -> PhysAddr;

    /// When mapping, all options should be reset to the same default state.
    fn map(&mut self, virt_addr: VirtAddr, phys_addr: PhysAddr) -> *mut u8;

    fn set_option(&mut self, virt_addr: VirtAddr, opt: VirtMemOptions) -> *mut u8;
    fn unset_option(&mut self, virt_addr: VirtAddr, opt: VirtMemOptions) -> *mut u8;
    fn read_option(
        &mut self,
        virt_addr: VirtAddr,
        opt: VirtMemOptions,
    ) -> Result<VirtMemError, bool>;

    fn set_options<T>(&mut self, virt_addr: VirtAddr, opts: T) -> *mut u8
    where
        T: Iterator<Item = VirtMemOptions>,
    {
        let mut ptr = null_mut();
        for opt in opts {
            ptr = self.set_option(virt_addr, opt)
        }
        ptr
    }

    fn unset_options<T>(&mut self, virt_addr: VirtAddr, opts: T) -> *mut u8
    where
        T: Iterator<Item = VirtMemOptions>,
    {
        let mut ptr = null_mut();
        for opt in opts {
            ptr = self.unset_option(virt_addr, opt)
        }
        ptr
    }
}
