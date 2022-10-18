use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Display, Formatter};

/// An Onyx identifier node.
// TODO: Wrapped ids.
#[derive(Clone, Debug)]
pub struct Id {
    span: Span,
    pub value: String,
}

impl Id {
    pub fn new(span: Span, value: String) -> Self {
        Self { span, value }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.value)
    }
}

impl HasSpan for Id {
    fn span(&self) -> Span {
        self.span
    }
}
