use core::{arch::asm, ops::Deref};

use core::fmt::Write;

use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

pub fn init_heap(
    mem_regions: &MemoryRegions,
    phys_mem_offet: u64,
) {
    let mut current = (0, 0);
    let mut biggest = (0, 0);
    for mem_region in mem_regions.deref() {
        if mem_region.kind == MemoryRegionKind::Usable {
            let (start, end) = (mem_region.start, mem_region.end);
            if start == current.1 {
                current.1 = end;
            } else {
                if current.1 - current.0 > biggest.1 - biggest.0 {
                    biggest = current;
                }
                current = (start, end);
            }
        }
    }
    if biggest.1 - biggest.0 < 2_109_440 {
        panic!()
    }

    let cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }

    let paging_virt_addr = phys_mem_offet + cr3;

    let mut l1_entry = 0;
    for i in 0..512 {
        unsafe {
            if ((paging_virt_addr + i * 8) as *mut u64).read() & 1 == 0 {
                l1_entry = i;
                break;
            }
        }
    }

    let page_2_phys = biggest.0;
    let page_3_phys = biggest.0 + 4096;
    let page_4_phys = biggest.0 + 4096 * 2;

    let heap_phys = biggest.0 + 4096 * 4;

    unsafe {
        let entry_ptr = (paging_virt_addr + l1_entry * 8) as *mut u64;
        *entry_ptr = page_2_phys | 0b11;

        let l2_entr_ptr = (phys_mem_offet + page_2_phys) as *mut u64;
        *l2_entr_ptr = page_3_phys | 0b11;

        let l3_entr_ptr = (phys_mem_offet + page_3_phys) as *mut u64;
        *l3_entr_ptr = page_4_phys | 0b11;

        for i in 0..512 {
            let l4_entry_ptr = (phys_mem_offet + page_4_phys + i * 8) as *mut u64;
            *l4_entry_ptr = (heap_phys + i * 4096) | 0b11;
        }

        let mut heap_ptr = l1_entry << (3 * 9 + 12);
        // sign extension
        // VERY important
        if heap_ptr & (1 << 47) != 0 {
            heap_ptr |= 0xFF_FF_00_00_00_00_00_00;
        }

        // test
        for i in 0..512 {
            *((heap_ptr + i * 4096) as *mut u8) = 8;
        }

        super::global_allocator::init_glob_alloc(
            heap_ptr as usize,
            (heap_ptr + 2_097_152) as usize,
        );
    }
}
