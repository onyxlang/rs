use super::{Comment, Expr, Statement};
use crate::location::{HasSpan, Span};
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum BlockBody {
    Comment(Comment),
    Stmt(Statement),
    Expr(Expr),
}

impl HasSpan for BlockBody {
    fn span(&self) -> Span {
        match self {
            BlockBody::Comment(c) => c.span(),
            BlockBody::Stmt(s) => s.span(),
            BlockBody::Expr(e) => e.span(),
        }
    }
}
