use std::fmt::{Display, Formatter};

use crate::location::{HasSpan, Span};

use super::{literal, Id};

/// A `use` node.
#[derive(Clone, Debug)]
pub struct Import {
    span: Span,
    pub r#pub: bool,
    pub ids: Vec<Id>,
    pub from: literal::String,
}

impl Import {
    pub fn new(span: Span, r#pub: bool, ids: Vec<Id>, from: literal::String) -> Self {
        Self {
            r#pub,
            span,
            ids,
            from,
        }
    }
}

impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        self.ids == other.ids && self.from == other.from
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "import {{{}}} from {}",
            self.ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.from
        )
    }
}

impl HasSpan for Import {
    fn span(&self) -> Span {
        self.span
    }
}
