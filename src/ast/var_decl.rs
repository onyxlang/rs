use super::{Expr, Id};
use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct VarDecl {
    span: Span,
    pub id: Id,
    pub expr: Expr,
}

impl VarDecl {
    pub fn new(span: Span, id: Id, expr: Expr) -> Self {
        Self { span, id, expr }
    }
}

impl PartialEq for VarDecl {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr
    }
}

impl Display for VarDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {}", self.id, self.expr)
    }
}

impl HasSpan for VarDecl {
    fn span(&self) -> Span {
        self.span
    }
}
