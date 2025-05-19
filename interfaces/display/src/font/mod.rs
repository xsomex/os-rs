pub mod monospace;

use alloc::{boxed::Box, vec::Vec};
use interfaces::{CallableObject, InterfaceInputOutput};

pub trait Font {
    fn char(&self, c: char) -> Vec<Vec<bool>>;
}

pub trait Char {
    fn bits(&self) -> Vec<Vec<bool>>;
}

pub struct GetChar;

impl InterfaceInputOutput for GetChar {
    type Input = char;
    type Output = Vec<Vec<bool>>;
}

impl CallableObject<GetChar> for Box<dyn Font> {
    fn call(&self, inputs: <GetChar as InterfaceInputOutput>::Input) -> <GetChar as InterfaceInputOutput>::Output {
        self.char(inputs)
    }
}
