use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub struct IndexError {
    index: usize,
    len: usize,
}

impl IndexError {
    pub fn new(index: usize, len: usize) -> Self {
        Self {
            index,
            len,
        }
    }
}

impl Display for IndexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Index {} out of bounds (len = {})", self.index, self.len)
    }
}

impl Error for IndexError {}
