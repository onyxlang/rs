use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct Cursor {
    pub offset: usize,
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl Cursor {
    pub fn new(offset: usize) -> Self {
        Self { offset }
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "&{}", self.offset)
    }
}

impl Debug for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "&{}", self.offset)
    }
}
