use super::{Expr, Id};
use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct MacroCall {
    span: Span,
    pub id: Id,
    pub args: Vec<Expr>,
}

impl MacroCall {
    pub fn new(span: Span, id: Id, args: Vec<Expr>) -> Self {
        Self { span, id, args }
    }
}

impl PartialEq for MacroCall {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.args == other.args
    }
}

impl Debug for MacroCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.id, self.args)
    }
}

impl HasSpan for MacroCall {
    fn span(&self) -> Span {
        self.span
    }
}
