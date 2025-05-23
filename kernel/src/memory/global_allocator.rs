use core::{
    alloc::{GlobalAlloc, Layout},
    cell::Cell,
    cmp::{max, min},
};

pub const WORD_SIZE: usize = size_of::<usize>();

pub struct GlobalAllocator {
    pub heap_start: Cell<usize>,
    pub heap_end: Cell<usize>,
}

unsafe impl Sync for GlobalAllocator {}

#[global_allocator]
pub static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator {
    heap_end: Cell::new(0),
    heap_start: Cell::new(0),
};

#[derive(Debug, Clone, Copy)]
pub struct UnusedRegion {
    pub address: usize,
    pub size: usize,
    pub next: Option<usize>,
    pub prev: Option<usize>,
}

impl UnusedRegion {
    pub const fn new() -> Self {
        UnusedRegion {
            address: 0,
            size: 0,
            next: None,
            prev: None,
        }
    }

    pub fn at_address(address: usize) -> Self {
        let mut size = 0;
        let mut next = 0;

        for i in 0..WORD_SIZE {
            size <<= 8;
            next <<= 8;

            size |= unsafe { ((address + i) as *mut u8).read() } as usize;
            next |= unsafe { ((address + WORD_SIZE + i) as *mut u8).read() } as usize;
        }

        let next = if next == 0 { None } else { Some(next) };

        UnusedRegion {
            address,
            size,
            next,
            prev: None,
        }
    }

    pub fn set_size(&mut self, mut new_size: usize) -> &mut Self {
        self.size = new_size;

        for i in (0..WORD_SIZE).rev() {
            unsafe { *((self.address + i) as *mut u8) = new_size as u8 };
            new_size >>= 8;
        }

        self
    }

    pub fn set_next(&mut self, mut new_next: usize) -> &mut Self {
        self.next = if new_next == 0 { None } else { Some(new_next) };

        for i in (0..WORD_SIZE).rev() {
            unsafe { *((self.address + WORD_SIZE + i) as *mut u8) = new_next as u8 };
            new_next >>= 8;
        }

        self
    }

    pub fn next_address(&self) -> usize {
        match self.next {
            None => 0,
            Some(v) => v,
        }
    }

    pub fn next(&self) -> Result<UnusedRegion, ()> {
        match self.next {
            None => Err(()),
            Some(next) => Ok({
                let mut r = UnusedRegion::at_address(next);
                r.prev = Some(self.address);
                r
            }),
        }
    }

    pub fn holds(&self, layout: Layout) -> Option<usize> {
        let mut skip = if self.address % layout.align() == 0 {
            0
        } else {
            layout.align() - (self.address % layout.align())
        };
        if skip != 0 && skip < 2 * WORD_SIZE {
            skip = 16 + layout.align() - ((self.address + 16) % layout.align());
        }

        if self.size - skip - 16 >= layout.size() {
            Some(self.address + skip)
        } else {
            None
        }
    }
}

impl GlobalAllocator {
    fn set_first_unused(&self, unused: UnusedRegion) {
        unsafe { *(self.heap_start.get() as *mut usize) = unused.address; }
    }
    pub fn get_first_unused(&self) -> UnusedRegion {
        let address = unsafe { (self.heap_start.get() as *mut usize).read() };
        UnusedRegion::at_address(address)
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let layout = unsafe {
            Layout::from_size_align_unchecked(max(2 * WORD_SIZE, layout.size()), layout.align())
        };
        let mut current_region = self.get_first_unused();

        loop {
            match current_region.holds(layout) {
                None => current_region = current_region.next().unwrap(),
                Some(ptr) => {
                    UnusedRegion::at_address(ptr + layout.size())
                        .set_size(
                            current_region.size + current_region.address - ptr - layout.size(),
                        )
                        .set_next(current_region.next_address());

                    if ptr == current_region.address {
                        match current_region.prev {
                            Some(prev) => {
                                UnusedRegion::at_address(prev).set_next(ptr + layout.size());
                            }
                            None => {
                                self.set_first_unused(UnusedRegion::at_address(ptr + layout.size()));
                            }
                        }
                    } else {
                        current_region
                            .set_size(ptr - current_region.address)
                            .set_next(ptr + layout.size());
                        self.set_first_unused(UnusedRegion::at_address(current_region.address));
                    }

                    return ptr as *mut u8;
                }
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let layout = unsafe {
            Layout::from_size_align_unchecked(max(2 * WORD_SIZE, layout.size()), layout.align())
        };
        let ptr = ptr as usize;
        let mut current_region = self.get_first_unused();

        loop {
            if current_region.address > ptr {
                match current_region.prev {
                    None => {
                        if ptr + layout.size() == current_region.address {
                            UnusedRegion::at_address(ptr)
                                .set_size(current_region.size + layout.size())
                                .set_next(current_region.next_address());
                        } else {
                            UnusedRegion::at_address(ptr)
                                .set_size(layout.size())
                                .set_next(current_region.address);
                        }
                        self.set_first_unused(UnusedRegion::at_address(ptr));
                    }
                    Some(prev) => {
                        let mut prev = UnusedRegion::at_address(prev);
                        if ptr + layout.size() == current_region.address {
                            if prev.address + prev.size == ptr {
                                prev.set_size(prev.size + layout.size() + current_region.size)
                                    .set_next(current_region.next_address());
                            } else {
                                UnusedRegion::at_address(ptr)
                                    .set_size(current_region.size + layout.size())
                                    .set_next(current_region.next_address());
                                prev.set_next(ptr);
                            }
                        } else {
                            if prev.address + prev.size == ptr {
                                prev.set_size(prev.size + layout.size());
                            } else {
                                UnusedRegion::at_address(ptr)
                                    .set_size(layout.size())
                                    .set_next(current_region.address);
                                prev.set_next(ptr);
                            }
                        }
                    }
                }

                return;
            } else {
                current_region = current_region.next().unwrap();
            }
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr =
            unsafe { self.alloc(Layout::from_size_align_unchecked(new_size, layout.align())) }
                as usize;
        let ptr = ptr as usize;

        for i in 0..min(layout.size(), new_size) {
            unsafe {
                *((new_ptr + i) as *mut u8) = ((ptr + i) as *mut u8).read();
            }
        }

        unsafe { self.dealloc(ptr as *mut u8, layout) };

        new_ptr as *mut u8
    }
}

pub fn init_glob_alloc(heap_start: usize, heap_end: usize) {
    GLOBAL_ALLOCATOR.heap_start.set(heap_start);
    GLOBAL_ALLOCATOR.heap_end.set(heap_end);
    let unused = *UnusedRegion::at_address(heap_start + WORD_SIZE)
        .set_size(heap_end - heap_start - WORD_SIZE)
        .set_next(0);

    GLOBAL_ALLOCATOR.set_first_unused(unused);
}

#[macro_export]
macro_rules! debug_global_allocator {
    ($display_text: expr) => {
        let mut current = GLOBAL_ALLOCATOR.get_first_unused();
        writeln!($display_text, "-------------------------");
        loop {
            writeln!(
                $display_text,
                "Begin: {}\nSize: {}",
                current.address, current.size
            );
            current = match current.next() {
                Ok(v) => v,
                _ => break,
            }
        }
    };
}
