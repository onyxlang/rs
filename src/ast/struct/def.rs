use crate::{
    ast::Id,
    location::{HasSpan, Span},
};
use std::fmt::{Debug, Display, Formatter};

/// A struct definition node.
#[derive(Clone)]
pub struct Def {
    span: Span,
    pub id: Id,
    pub export: bool,
    pub default: bool,
}

impl Def {
    // TODO: Panic if default without export.
    pub fn new(span: Span, id: Id, export: bool, default: bool) -> Self {
        Self {
            span,
            id,
            export,
            default,
        }
    }
}

impl PartialEq for Def {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Debug for Def {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StructDef{{id: {:?}}}", self.id)
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
