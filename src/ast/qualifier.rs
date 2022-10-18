use std::fmt::Display;

use super::Id;
use crate::location::{HasSpan, Span};

#[derive(PartialEq, Debug, Clone)]
pub struct Qualifier {
    span: Span,
    // TODO: pub container: Option<Box<Qualifier>>,
    // TODO: pub accessor: Option<Accessor>,
    pub id: Id,
}

impl Qualifier {
    pub fn new(span: Span, id: Id) -> Self {
        Self { span, id }
    }

    pub fn from_string(span: Span, id: String) -> Self {
        Self::new(span, Id::new(span, id))
    }
}

impl HasSpan for Qualifier {
    fn span(&self) -> Span {
        self.span
    }
}

impl Display for Qualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}
