use super::{Expr, Id};
use crate::location::{HasSpan, Span};

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    span: Span,
    pub callee: Id,
    pub args: Vec<Expr>,
}

impl Call {
    pub fn new(span: Span, callee: Id, args: Vec<Expr>) -> Self {
        Self { span, callee, args }
    }
}

impl HasSpan for Call {
    fn span(&self) -> Span {
        self.span
    }
}
