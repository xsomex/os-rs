// TO BE DELETED
// DEBUG PURPOSES ONLY

use core::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeaplessVec<T: Copy + PartialEq, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct HeaplessVecIter<T: Copy + PartialEq, const N: usize> {
    vec: HeaplessVec<T, N>,
    index: usize,
}

impl<T: Copy + PartialEq, const N: usize> HeaplessVec<T, N> {
    pub const fn new() -> Self {
        HeaplessVec {
            data: [None; N],
            len: 0,
        }
    }

    pub const fn push(&mut self, value: T) {
        if self.len < N - 1 {
            self.len += 1;
            self.data[self.len - 1] = Some(value);
        } else {
            panic!()
        }
    }

    pub fn index_of(&self, value: T) -> Option<usize> {
        for i in 0..self.len {
            if let Some(v) = self.data[i] {
                if v == value {
                    return Some(i);
                }
            }
        }
        None
    }
}

impl<T: Copy + PartialEq, const N: usize> Index<usize> for HeaplessVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.len {
            if let Some(value) = &self.data[index] {
                return value;
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

impl<T: Copy + PartialEq, const N: usize> IndexMut<usize> for HeaplessVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.len {
            if let Some(value) = &mut self.data[index] {
                return value;
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

impl<T: Copy + PartialEq, const N: usize> IntoIterator for HeaplessVec<T, N> {
    type Item = T;
    type IntoIter = HeaplessVecIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        HeaplessVecIter {
            vec: self,
            index: 0,
        }
    }
}

impl<T: Copy + PartialEq, const N: usize> Iterator for HeaplessVecIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < N {
            self.index += 1;
            self.vec.data[self.index - 1]
        } else {
            None
        }
    }
}
