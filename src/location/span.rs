use std::fmt::{Debug, Display, Formatter};

use super::Cursor;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Cursor,
    pub end: Cursor,
}

impl Span {
    pub fn new(start: Cursor, end: Cursor) -> Self {
        Self { start, end }
    }

    pub fn zero() -> Self {
        Self::new(Cursor::new(0), Cursor::new(0))
    }

    pub fn thin(cursor: Cursor) -> Self {
        Self::new(cursor, cursor)
    }

    pub fn join(self, other: Self) -> Self {
        Self::new(self.start, other.end)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}..{}", self.start, self.end)
        }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "{:?}", self.start)
        } else {
            write!(f, "{:?}..{:?}", self.start, self.end)
        }
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}
