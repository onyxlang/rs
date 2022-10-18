use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Display, Formatter};

/// A string literal node.
#[derive(Clone, Debug)]
pub struct String {
    span: Span,
    pub value: std::string::String,
}

impl String {
    pub fn new(span: Span, value: std::string::String) -> Self {
        Self { span, value }
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl HasSpan for String {
    fn span(&self) -> Span {
        self.span
    }
}
