use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Formatter};

/// A generic text node.
#[derive(Clone)]
pub struct Node {
    span: Span,
    pub text: String,
}

impl Node {
    pub fn new(span: Span, text: String) -> Self {
        Self { span, text }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl HasSpan for Node {
    fn span(&self) -> Span {
        self.span
    }
}
