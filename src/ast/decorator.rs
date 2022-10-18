use super::Id;
use crate::location::Span;
use std::fmt::Display;

/// A decorator node, e.g. `@[Builtin]`.
#[derive(Clone, Debug)]
pub struct Decorator {
    span: Span,
    pub id: Id,
}

impl Decorator {
    pub fn new(span: Span, id: Id) -> Self {
        Self { span, id }
    }
}

impl PartialEq for Decorator {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Decorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@[{}]", self.id)
    }
}

impl crate::location::HasSpan for Decorator {
    fn span(&self) -> Span {
        self.span
    }
}
