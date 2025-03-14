#[repr(transparent)]
pub struct PhysAddr(u64);


pub enum PhysMemState {
    /// Optional: the number of times this block is used
    Used(Option<usize>),

    /// Block is not used
    Free,
}

pub trait PhysMemManager {
    fn find_free(&mut self, size: usize) -> PhysAddr;
    fn state_of(&mut self, addr: PhysAddr) -> PhysMemState;

    fn set_used(&mut self);
    fn set_unused(&mut self);
}
