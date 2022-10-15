use std::fmt::{Debug, Display, Formatter};

pub struct Cursor {
    pub origin: super::super::Cursor,
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new(origin: super::super::Cursor, src: &str) -> Self {
        let before = &src[..origin.offset];

        let line = before.as_bytes().iter().filter(|&&c| c == b'\n').count();
        let column = before.chars().rev().take_while(|&c| c != '\n').count();

        Self {
            origin,
            line,
            column,
        }
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}

impl Debug for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}
