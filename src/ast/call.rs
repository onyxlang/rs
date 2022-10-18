use std::fmt::Display;

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

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.callee)?;

        for (i, e) in self.args.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", e)?;
        }

        write!(f, ")")
    }
}
