use std::fmt::{Debug, Display, Formatter};

use super::Cursor;

pub struct Span {
    pub origin: super::super::Span,
    pub start: Cursor,
    pub end: Cursor,
}

impl Span {
    pub fn new(origin: super::super::Span, src: &str) -> Self {
        Self {
            origin,
            start: Cursor::new(origin.start, src),
            end: Cursor::new(origin.end, src),
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
