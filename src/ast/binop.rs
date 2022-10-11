use super::Expr;
use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Binop {
    span: Span,
    pub lhs: Box<Expr>,

    /// ADHOC: Can not have `Node` here due to `peg` limitations:
    /// a labeled capture is not supported within a `precedence!` macro.
    pub op: String,

    pub rhs: Box<Expr>,
}

impl Binop {
    pub fn new(lhs: Expr, op: String, rhs: Expr) -> Self {
        Self {
            span: lhs.span().join(rhs.span()),
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl PartialEq for Binop {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.op == other.op && self.rhs == other.rhs
    }
}

impl Debug for Binop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?}",
            self.lhs.as_ref(),
            self.op,
            self.rhs.as_ref()
        )
    }
}

impl HasSpan for Binop {
    fn span(&self) -> Span {
        self.span
    }
}
