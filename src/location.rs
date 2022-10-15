use std::{
    fmt::{Debug, Display, Formatter},
    path::PathBuf,
};

#[derive(Clone, Copy, Eq)]
pub struct Cursor {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl Cursor {
    pub fn new(offset: usize, line: usize, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }

    pub fn incomplete(offset: usize) -> Self {
        Self::new(offset, 0, 0)
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// ADHOC: This is a hack to make the cursor work
    /// with the parser, which only yields offsets.
    pub fn is_incomplete(&self) -> bool {
        self.offset != 0 && self.line == 0 && self.column == 0
    }

    /// ADHOC: This is a hack to make the cursor work
    /// with the parser, which only yields offsets.
    pub fn complete(self, src: &str) -> Self {
        let before = &src[..self.offset];
        let line = before.as_bytes().iter().filter(|&&c| c == b'\n').count();
        let column = before.chars().rev().take_while(|&c| c != '\n').count();
        Self::new(self.offset, line, column)
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}

impl Debug for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_incomplete() {
            write!(f, "&{}", self.offset)
        } else {
            write!(f, "{}:{}", self.line + 1, self.column + 1)
        }
    }
}

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
        Self::new(Cursor::zero(), Cursor::zero())
    }

    pub fn thin(cursor: Cursor) -> Self {
        Self::new(cursor, cursor)
    }

    pub fn incomplete(start_offset: usize, end_offset: usize) -> Self {
        Self::new(
            Cursor::incomplete(start_offset),
            Cursor::incomplete(end_offset),
        )
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

#[derive(Clone)]
pub struct Location {
    pub path: PathBuf,
    pub span: Span,
}

impl Location {
    pub fn new(path: PathBuf, mut span: Span) -> Self {
        if span.start.is_incomplete() || span.end.is_incomplete() {
            let src = std::fs::read_to_string(&path).unwrap();
            span.start = span.start.complete(&src);
            span.end = span.end.complete(&src);
        }

        Self { path, span }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // BUG: Breaks if path contains invalid Unicode.
        write!(f, "{}:{}", self.path.as_path().to_str().unwrap(), self.span)
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
