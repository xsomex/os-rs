pub mod monospace;

use core::{char, isize};
use crate::memory::vec::HeaplessVec as Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Char<const W: usize, const H: usize> {
    size: (usize, usize),
    /// position of the top-left corner relatively to the main line
    position: (isize, isize),
    bytes: Vec<Vec<bool, W>, H>,
}

/// prefix: 'c_'
pub struct Font {
    chars: Vec<char, 256>,
    bytes: Vec<Char<32, 32>, 256>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

pub enum FontsList {
    Monospace,
}

impl Font {
    pub fn char(&self, c: char) -> Option<Char<32, 32>> {
        let index = self.chars.index_of(c);
        if let Some(i) = index {
            Some(self.bytes[i])
        } else {
            None
        }
    }
}

impl<const W: usize, const H: usize> Char<W, H> {
    pub fn bytes(&self) -> Vec<Vec<bool, W>, H> {
        self.bytes
    }
    pub fn position(&self) -> (isize, isize) {
        self.position
    }
}

#[macro_export]
macro_rules! b_vec {
    ( $( $b: expr),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($b);
            )*
            v
        }
    };
}

#[macro_export]
macro_rules! add_char {
    ( $font: expr, $char: expr, $size: expr, $position: expr, $bytes: expr ) => {
        $font.chars.push($char);
        $font.bytes.push( Char {
            size: $size,
            position: $position,
            bytes: $bytes
        });
    };
}
