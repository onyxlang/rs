use super::{literal, Id};
use crate::location::{HasSpan, Span};

/// A freestanding `export` statement.
#[derive(PartialEq, Debug)]
pub struct Export {
    span: Span,
    pub id: Id,
    pub from: literal::String,
}

impl Export {
    pub fn new(span: Span, id: Id, from: literal::String) -> Self {
        Self { span, id, from }
    }
}

impl HasSpan for Export {
    fn span(&self) -> Span {
        self.span
    }
}
