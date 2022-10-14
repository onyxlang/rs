use std::fmt::{Debug, Display, Formatter};

use crate::location::{HasSpan, Span};

use super::{literal, Id};

/// An import node.
#[derive(Clone)]
pub struct Import {
    span: Span,
    pub id: Id,
    pub from: literal::String,
}

impl Import {
    pub fn new(span: Span, id: Id, from: literal::String) -> Self {
        Self { span, id, from }
    }
}

impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.from == other.from
    }
}

impl Debug for Import {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Import{{id: {:?}, from: {:?}}}", self.id, self.from)
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "import {} from {}", self.id, self.from)
    }
}

impl HasSpan for Import {
    fn span(&self) -> Span {
        self.span
    }
}
