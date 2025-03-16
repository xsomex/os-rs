#[repr(align(8))]
#[derive(Debug, Clone, Copy)]
pub struct TableEntry(u64);

#[derive(Debug, Clone, Copy)]
pub struct PageTable([TableEntry; 512]);

#[derive(Debug, Clone, Copy)]
pub struct VirtMemManager {
    level_4: *mut PageTable,
    extend_beginning: u64,
    extend_size: usize,
}

impl modules_common::InitModule for VirtMemManager {
    fn init(boot_info: &mut bootloader_api::info::BootInfo) {
        
    }
}
