use core::{
    alloc::{GlobalAlloc, Layout},
    cmp::max,
    usize,
};
pub const WORD_SIZE: usize = size_of::<usize>();

#[derive(Debug, Clone, Copy)]
pub struct UnusedRegion {
    address: usize,
    size: usize,
    next_unused_region: Option<usize>,
    previous_unused_region: Option<usize>,
}

impl UnusedRegion {
    pub fn at_address(address: usize) -> Self {
        let mut size = 0;
        let mut next_unused_region = 0;
        for i in 0..WORD_SIZE {
            size |= unsafe {
                (((address + i) as *mut u8).read() as usize) << (((WORD_SIZE - i) * 8) % WORD_SIZE)
            };
            next_unused_region |= unsafe {
                (((address + WORD_SIZE + i) as *mut u8).read() as usize)
                    << (((WORD_SIZE - i) * 8) % WORD_SIZE)
            };
        }

        let next_unused_region = if next_unused_region == 0 {
            None
        } else {
            Some(next_unused_region)
        };

        UnusedRegion {
            address,
            size,
            next_unused_region,
            previous_unused_region: None,
        }
    }

    pub fn set_size(&mut self, new_size: usize) -> &mut UnusedRegion {
        for i in 0..WORD_SIZE {
            unsafe {
                *((self.address + i) as *mut u8) =
                    (new_size >> (((WORD_SIZE - i) * 8) % WORD_SIZE)) as u8;
            }
        }
        self.size = new_size;

        self
    }

    pub fn set_next_unused_region(&mut self, new_next_unused_region: usize) -> &mut UnusedRegion {
        for i in WORD_SIZE..2 * WORD_SIZE {
            unsafe {
                *((self.address + i) as *mut u8) =
                    (new_next_unused_region >> (((2 * WORD_SIZE - i) * 8) % WORD_SIZE)) as u8;
            }
        }
        self.next_unused_region = if new_next_unused_region == 0 {
            None
        } else {
            Some(new_next_unused_region)
        };

        self
    }

    pub fn next(&self) -> Result<UnusedRegion, ()> {
        match self.next_unused_region {
            Some(ptr) => Ok({
                let mut r = UnusedRegion::at_address(ptr);
                r.previous_unused_region = Some(self.address);
                r
            }),
            _ => Err(()),
        }
    }

    pub fn holds(&self, layout: Layout) -> Option<usize> {
        let mut skip = layout.align() - (self.address % layout.align());
        if skip != 0 && skip < 2 * WORD_SIZE {
            skip =
                2 * WORD_SIZE + layout.align() - ((self.address + 2 * WORD_SIZE) % layout.align());
        }
        if self.size - skip - 2 * WORD_SIZE >= layout.size() {
            Some(self.address + skip)
        } else {
            None
        }
    }

    pub const fn size(&self) -> usize {
        self.size
    }
    pub const fn address(&self) -> usize {
        self.address
    }
    pub const fn next_unused_region(&self) -> Option<usize> {
        self.next_unused_region
    }
}

pub struct GlobalAllocator {
    pub heap_start: usize,
    pub heap_end: usize,
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut current_unused_region =
            UnusedRegion::at_address(unsafe { (self.heap_start as *mut usize).read() });
        let layout =
            Layout::from_size_align(max(2 * WORD_SIZE, layout.size()), layout.align()).unwrap();

        loop {
            match current_unused_region.holds(layout) {
                Some(ptr) => {
                    UnusedRegion::at_address(ptr + layout.size())
                        .set_size(
                            current_unused_region.address() + current_unused_region.size()
                                - ptr
                                - layout.size(),
                        )
                        .set_next_unused_region(match current_unused_region.next_unused_region() {
                            Some(v) => v,
                            _ => 0,
                        });

                    if ptr == current_unused_region.address() {
                        match current_unused_region.previous_unused_region {
                            Some(prev) => {
                                UnusedRegion::at_address(prev)
                                    .set_next_unused_region(ptr + layout.size());
                            }
                            None => unsafe {
                                *(self.heap_start as *mut usize) = ptr + layout.size()
                            },
                        }
                    } else {
                        current_unused_region
                            .set_size(
                                current_unused_region.address() + current_unused_region.size()
                                    - ptr,
                            )
                            .set_next_unused_region(ptr + layout.size());
                    }

                    return ptr as *mut u8;
                }
                None => {
                    current_unused_region = current_unused_region.next().unwrap();
                }
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let ptr = ptr as usize;
        let layout =
            Layout::from_size_align(max(2 * WORD_SIZE, layout.size()), layout.align()).unwrap();
        let mut current_unused_region =
            UnusedRegion::at_address(unsafe { (self.heap_start as *mut usize).read() });

        loop {
            if ptr < current_unused_region.address() {
                if ptr + layout.size() == current_unused_region.address() {
                    match current_unused_region.previous_unused_region {
                        Some(prev) => {
                            let mut prev = UnusedRegion::at_address(prev);
                            if prev.address() + prev.size() == ptr {
                                prev.set_size(
                                    prev.size() + layout.size() + current_unused_region.size(),
                                )
                                .set_next_unused_region(
                                    match current_unused_region.next_unused_region() {
                                        Some(v) => v,
                                        None => 0,
                                    },
                                );
                            } else {                                
                                UnusedRegion::at_address(ptr)
                                    .set_size(current_unused_region.size() + layout.size())
                                    .set_next_unused_region(
                                        match current_unused_region.next_unused_region() {
                                            Some(v) => v,
                                            None => 0,
                                        },
                                    );
                                prev.set_next_unused_region(ptr);
                            }
                        }
                        None => {
                            UnusedRegion::at_address(ptr)
                                .set_size(current_unused_region.size() + layout.size())
                                .set_next_unused_region(
                                    match current_unused_region.next_unused_region() {
                                        Some(v) => v,
                                        None => 0,
                                    },
                                );
                            unsafe { *(self.heap_start as *mut usize) = ptr };
                        }
                    }
                } else {
                    match current_unused_region.previous_unused_region {
                        Some(prev) => {
                            let mut prev = UnusedRegion::at_address(prev);
                            if prev.address() + prev.size() == ptr {
                                prev.set_size(
                                    prev.size() + layout.size(),
                                )
                                .set_next_unused_region(current_unused_region.address());
                            } else {                                
                                UnusedRegion::at_address(ptr)
                                    .set_size(layout.size())
                                    .set_next_unused_region(current_unused_region.address());
                                prev.set_next_unused_region(ptr);
                            }
                        }
                        None => {
                            UnusedRegion::at_address(ptr)
                                .set_size(layout.size())
                                .set_next_unused_region(current_unused_region.address());
                            unsafe { *(self.heap_start as *mut usize) = ptr };
                        }
                    }
                }

                return;
            }

            current_unused_region = current_unused_region.next().unwrap();
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        0 as *mut u8
    }
}

#[global_allocator]
pub static mut GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator {
    heap_start: 0,
    heap_end: 0,
};

pub fn init_glob_alloc(heap_start: usize, heap_end: usize) {
    unsafe {
        GLOBAL_ALLOCATOR.heap_start = heap_start;
        GLOBAL_ALLOCATOR.heap_end = heap_end;
        UnusedRegion::at_address(heap_start + WORD_SIZE)
            .set_size(heap_end - heap_start - WORD_SIZE)
            .set_next_unused_region(0);
        *(heap_start as *mut usize) = heap_start + WORD_SIZE;
    }
}
