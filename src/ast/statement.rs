use super::{function, r#struct, Decorator, Expr, Import, VarDecl};
use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Display};

#[derive(PartialEq, Debug)]
pub enum Statement {
    VarDecl(VarDecl),
    TerminatedExpr(Expr),
    Import(Import),

    // IDEA: An expression may also be decorated?
    Decorator(Decorator),

    StructDef(r#struct::Def),
    FunctionDecl(function::Decl),
}

impl HasSpan for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::VarDecl(v) => v.span(),
            Statement::TerminatedExpr(e) => e.span(),
            Statement::Import(i) => i.span(),
            Statement::Decorator(d) => d.span(),
            Statement::StructDef(d) => d.span(),
            Statement::FunctionDecl(d) => d.span(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VarDecl(v) => write!(f, "{}", v),
            Statement::TerminatedExpr(e) => write!(f, "{}", e),
            Statement::Import(i) => write!(f, "{}", i),
            Statement::Decorator(d) => write!(f, "{}", d),
            Statement::StructDef(d) => write!(f, "{}", d),
            Statement::FunctionDecl(d) => write!(f, "{}", d),
        }
    }
}
