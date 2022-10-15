use super::{function, r#struct, Decorator, Export, Expr, Import, VarDecl};
use crate::location::{HasSpan, Span};
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum Statement {
    VarDecl(VarDecl),
    TerminatedExpr(Expr),
    Import(Import),
    Export(Export),

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
            Statement::Export(e) => e.span(),
            Statement::Decorator(d) => d.span(),
            Statement::StructDef(d) => d.span(),
            Statement::FunctionDecl(d) => d.span(),
        }
    }
}
