use super::{literal, Binop, Id, MacroCall};
use crate::location::{HasSpan, Span};
use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    BoolLiteral(literal::Bool),
    IdRef(Id),
    MacroCall(MacroCall),
    Binop(Binop),
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::BoolLiteral(b) => b.span(),
            Expr::IdRef(id) => id.span(),
            Expr::MacroCall(m) => m.span(),
            Expr::Binop(b) => b.span(),
        }
    }
}
