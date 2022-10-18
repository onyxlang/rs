use super::{literal, Binop, Call, MacroCall, Qualifier};
use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Display};

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    BoolLiteral(literal::Bool),
    Ref(Qualifier),
    MacroCall(MacroCall),
    Binop(Binop),
    FunctionCall(Call),
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::BoolLiteral(b) => b.span(),
            Expr::Ref(id) => id.span(),
            Expr::MacroCall(m) => m.span(),
            Expr::Binop(b) => b.span(),
            Expr::FunctionCall(c) => c.span(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::BoolLiteral(b) => write!(f, "{}", b),
            Expr::Ref(id) => write!(f, "{}", id),
            Expr::MacroCall(m) => write!(f, "{}", m),
            Expr::Binop(b) => write!(f, "{}", b),
            Expr::FunctionCall(c) => write!(f, "{}", c),
        }
    }
}
