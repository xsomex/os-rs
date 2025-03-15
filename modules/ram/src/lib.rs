#![no_std]
//! This module provides some primitives to deal with memory. It will content a physical memory
//! manager, which represents the physical memory into virtual one, and a virtual memory manager,
//! which provides an interface to allocate memory, find available blocks, ...

pub struct VirtAddr(u64);
pub struct PhysAddr(u64);

/// TODO: 
#[repr(u64)]
pub enum BlockOption {
    TODO = 0,
}

pub trait Ram
where
    Self: modules_common::InitModule,
{
    fn find_free_virtual(&mut self, size: usize) -> Result<(), VirtAddr>;
    fn find_free_physical(&mut self, size: usize) -> Result<(), PhysAddr>;
    fn map<T>(
        &mut self,
        phys_addr: PhysAddr,
        virt_addr: VirtAddr,
        size: usize,
        options: T,
    ) -> &mut u8
    where
        T: Iterator<Item = BlockOption>;
    // guarantee to be from virt_addr to virt_addr + 4096*size, but the physical memory is not
    // guarantee to be continuous
    fn map_free_phys_to(&mut self, virt_addr: VirtAddr, size: usize) -> *mut u8;
    // only maps one block.
    // TODO: implement vec
    fn map_free_virt_to(&mut self, phys_addr: PhysAddr) -> *mut u8;
}
