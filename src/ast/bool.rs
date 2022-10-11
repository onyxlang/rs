use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Formatter};

/// A boolean literal node.
#[derive(Clone)]
pub struct Bool {
    span: Span,
    pub value: bool,
}

impl Bool {
    pub fn new(span: Span, value: bool) -> Self {
        Self { span, value }
    }
}

impl PartialEq for Bool {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Debug for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl HasSpan for Bool {
    fn span(&self) -> Span {
        self.span
    }
}
