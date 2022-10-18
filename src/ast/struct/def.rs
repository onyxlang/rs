use crate::{
    ast::Id,
    location::{HasSpan, Span},
};
use std::fmt::{Debug, Display, Formatter};

/// A struct definition node.
#[derive(Clone, Debug)]
pub struct Def {
    span: Span,
    pub r#pub: bool,
    pub id: Id,
}

impl Def {
    // TODO: Panic if default without export.
    pub fn new(span: Span, r#pub: bool, id: Id) -> Self {
        Self { span, r#pub, id }
    }
}

impl PartialEq for Def {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Def {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct {} {{}}", self.id)
    }
}

impl HasSpan for Def {
    fn span(&self) -> Span {
        self.span
    }
}
