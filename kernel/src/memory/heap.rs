use core::ops::Deref;

use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

pub fn init_heap(mem_regions: MemoryRegions, recursive_index: u16) {
    for mem_region in mem_regions.deref() {
        if mem_region.kind == MemoryRegionKind::Usable {
            let (start, end) = (mem_region.start, mem_region.end);

        }
    }
}
