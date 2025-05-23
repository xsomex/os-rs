pub mod monospace;

use alloc::vec::Vec;
use crate::objects::ObjectFunction;

pub trait Char {
    fn bits(&self) -> Vec<Vec<bool>>;
}

pub struct Font;

pub struct GetChar;
impl ObjectFunction for GetChar {
    type Input = char;
    type Output = Vec<Vec<bool>>;
}
