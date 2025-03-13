#[repr(transparent)]
pub struct PhysAddr(u64);

pub trait PhysMemManager {
    fn find_free(&mut self, size: usize) -> u64;
}
