use super::{Expr, VarDecl};
use crate::location::{HasSpan, Span};
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum Statement {
    VarDecl(VarDecl),
    TerminatedExpr(Expr),
}

impl HasSpan for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::VarDecl(v) => v.span(),
            Statement::TerminatedExpr(e) => e.span(),
        }
    }
}
