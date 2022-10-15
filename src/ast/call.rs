use super::{Expr, Qualifier};
use crate::location::{HasSpan, Span};

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    span: Span,
    pub callee: Qualifier,
    pub args: Vec<Expr>,
}

impl Call {
    pub fn new(span: Span, callee: Qualifier, args: Vec<Expr>) -> Self {
        Self { span, callee, args }
    }
}

impl HasSpan for Call {
    fn span(&self) -> Span {
        self.span
    }
}
