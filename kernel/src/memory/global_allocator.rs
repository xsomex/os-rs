use core::{alloc::{GlobalAlloc, Layout}, cmp::max};

pub struct GlobalAllocator {
    heap_start: u64,
    heap_end: u64,
}

// TODO:
// Find WHY TH F**K THERE IS BUG WHEN DEALLOCATING VEC WITH NEW SIZE
//
// When Vec is reallocated, then deallocate Vec fails
// Is it a problem of allocate, deallocate or reallocate?
// TODO: Find a way to debug this F**KING CODE!
//
// Probably deallocate -> whithout drop(), everything works
unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = max(layout.size() as u64, 16);
        let align = max(layout.align() as u64, 16);

        let first_unused = unsafe { (self.heap_start as *mut u64).read() };
        let mut current_unused_address = first_unused;
        let mut precedent_unused_address = 0;

        loop {
            if current_unused_address == 0 {
                panic!()
            }

            let unused_info = unsafe { (current_unused_address as *mut u128).read() };
            let unused_size = (unused_info >> 64) as u64;

            let mut skip_for_align = align - current_unused_address % align;
            if skip_for_align != 0 && skip_for_align < 16 {
                skip_for_align = 16 + align - ((current_unused_address + 16) % align);
            }
            if unused_size - skip_for_align - 16 >= size {
                let ptr = (current_unused_address + skip_for_align) as *mut u8;
                unsafe {
                    *(current_unused_address as *mut u128) =
                        ((current_unused_address + skip_for_align + size) as u128) | ((skip_for_align as u128) << 64);
                    *((current_unused_address + skip_for_align + size) as *mut u128) = (unused_info
                        & 0xFFFF_FFFF_FFFF_FFFF)
                        | (((unused_size - skip_for_align - size) as u128) << 64);
                }
                if skip_for_align == 0 && current_unused_address == first_unused {
                    unsafe {
                        *(self.heap_start as *mut u64) = current_unused_address + skip_for_align + size
                    };
                } else if skip_for_align == 0 {
                    unsafe {
                        *(precedent_unused_address as *mut u128) &= !0xFFFF_FFFF_FFFF_FFFF;
                        *(precedent_unused_address as *mut u128) |= (current_unused_address + skip_for_align + size) as u128;
                    }
                }
                return ptr;
            }

            precedent_unused_address = current_unused_address;
            current_unused_address = (unused_info & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let first_unused = unsafe { (self.heap_start as *mut u64).read() };
        let mut current_unused_address = first_unused;
        let ptr = ptr as u64;

        if ptr < first_unused {
            let unused = unsafe { (current_unused_address as *mut u128).read() };
            let unused_size = (unused >> 64) as u64;
            let next_unused = (unused & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            if ptr + max(16, layout.size() as u64) == first_unused {
                unsafe {
                    *(ptr as *mut u128) = (((unused_size + max(16, layout.size() as u64)) as u128)
                        << 64)
                        | (next_unused as u128);
                }
            } else {
                unsafe {
                    *(ptr as *mut u128) =
                        (((max(16, layout.size() as u64)) as u128) << 64) | (first_unused as u128)
                };
            }
            unsafe { *(self.heap_start as *mut u64) = ptr };
        }

        loop {
            if current_unused_address == 0 {
                break;
            }
            let unused = unsafe { (current_unused_address as *mut u128).read() };
            let unused_size = (unused >> 64) as u64;
            let next_unused = (unused & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            if ptr > current_unused_address && ptr <= next_unused {
                if ptr == current_unused_address + unused_size {
                    if ptr + max(16, layout.size() as u64) == next_unused {
                        unsafe {
                            let next_size = ((next_unused as *mut u128).read() >> 64) as u64;
                            let after_next = ((next_unused as *mut u128).read()) as u64;
                            *(current_unused_address as *mut u128) =
                                ((unused_size + next_size + max(16, layout.size() as u64)) as u128)
                                    | (after_next as u128);
                        }
                    } else {
                        unsafe {
                            *(current_unused_address as *mut u128) =
                                (((unused_size + max(16, layout.size() as u64)) as u128) << 64)
                                    | (next_unused as u128);
                        }
                    }
                } else {
                    if ptr + max(16, layout.size() as u64) == next_unused {
                        unsafe {
                            let next_size = ((next_unused as *mut u128).read() >> 64) as u64;
                            let after_next = ((next_unused as *mut u128).read()) as u64;
                            *(ptr as *mut u128) = ((next_size + max(16, layout.size() as u64))
                                as u128)
                                | (after_next as u128);
                        }
                    } else {
                        unsafe {
                            *(ptr as *mut u128) = (((max(16, layout.size() as u64)) as u128) << 64)
                                | (next_unused as u128)
                        };
                    }
                    unsafe {
                        *(current_unused_address as *mut u128) =
                            ((unused_size as u128) << 64) | (ptr as u128)
                    };
                }
            }

            current_unused_address = (unused & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        unsafe {
            let ptr = ptr as u64;
            let new_ptr = self.alloc(Layout::from_size_align(new_size, layout.align()).unwrap()) as u64;
            for i in 0..(layout.size()as u64) {
                *((new_ptr + i) as *mut u8) = ((ptr + i) as *mut u8).read();
            }
            self.dealloc(ptr as *mut u8, layout);
            new_ptr as *mut u8
        }
    }
}

#[global_allocator]
pub static mut GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator {
    heap_start: 0,
    heap_end: 0,
};

pub fn init_glob_alloc(heap_start: u64, heap_end: u64) {
    unsafe {
        GLOBAL_ALLOCATOR.heap_start = heap_start;
        GLOBAL_ALLOCATOR.heap_end = heap_end;
        *((heap_start + 16) as *mut u128) = ((heap_end - heap_start) as u128) << 64;
        *(heap_start as *mut u64) = heap_start + 16;
    }
}
