use std::{cell::RefCell, rc::Rc};

use super::{r#struct, Assignment, Call, Exportable, InferType, MacroCall, Scope, VarRef};
use crate::{
    ast::{self},
    location::{HasSpan, Span},
};

#[derive(Debug)]
pub enum Expr {
    BoolLiteral(ast::literal::Bool),
    VarRef(VarRef),
    MacroCall(MacroCall),
    FunctionCall(Call),
    Assignment(Assignment),
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::BoolLiteral(b) => b.span(),
            Expr::VarRef(r) => r.span(),
            Expr::MacroCall(m) => m.span(),
            Expr::Assignment(a) => a.span(),
            Expr::FunctionCall(c) => c.span(),
        }
    }
}

impl InferType for Expr {
    fn infer_type(&self, scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>> {
        match self {
            Expr::BoolLiteral(_) => {
                let found = scope.search_builtin("Bool");

                if let Some(Exportable::StructDecl(decl)) = found {
                    Some(decl)
                } else {
                    panic!("`Bool` not found")
                }
            }
            Expr::VarRef(r) => r.infer_type(scope),
            Expr::MacroCall(m) => m.infer_type(scope),
            Expr::Assignment(a) => a.infer_type(scope),
            Expr::FunctionCall(c) => c.infer_type(scope),
        }
    }
}
