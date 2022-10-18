use super::{Expr, Id};
use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
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

impl Display for MacroCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}(", self.id)?;

        for (i, e) in self.args.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", e)?;
        }

        write!(f, ")")
    }
}

impl HasSpan for MacroCall {
    fn span(&self) -> Span {
        self.span
    }
}
