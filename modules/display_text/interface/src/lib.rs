#![no_std]

#[derive(Debug)]
pub struct IndexOutOfRange;

pub enum ReturnMode {
    None,
    FollowText,
    SameLine,
    SameCol,
}

pub trait DisplayText
where
    Self: core::fmt::Write,
{
    fn set_foreground_color(&mut self, color: (u8, u8, u8));
    fn set_background_color(&mut self, color: (u8, u8, u8));
    fn fill(&mut self, color: (u8, u8, u8));
    fn goto(&mut self, x: usize, y: usize) -> Result<(), IndexOutOfRange>;

    fn move_left(&mut self, count: usize, return_mode: ReturnMode) -> Result<(), IndexOutOfRange>;
    fn move_right(&mut self, count: usize, return_mode: ReturnMode) -> Result<(), IndexOutOfRange>;
    fn move_up(&mut self, count: usize, return_mode: ReturnMode) -> Result<(), IndexOutOfRange>;
    fn move_down(&mut self, count: usize, return_mode: ReturnMode) -> Result<(), IndexOutOfRange>;
}
