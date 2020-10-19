use std::fmt;
use std::fmt::{Formatter,};

pub(crate) struct List(Vec<i32>);

impl List {
    pub(crate) fn new(vec: Vec<i32>) -> List {
        List(vec)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count,v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{} --> {}", count, v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)] //{:?}
pub (crate) struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ")?;
        write!(f, "({}, {}, {}) ", self.red, self.green, self.blue)?;
        write!(f, "{:#X}{:X}{:X}", self.red, self.green, self.blue)
    }
}
