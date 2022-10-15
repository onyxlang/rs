use super::{r#struct, Expr, InferType, Scope, VarDecl};
use std::{cell::RefCell, rc::Rc};

pub enum Statement {
    VarDecl(Rc<VarDecl>),
    TerminatedExpr(Rc<Expr>),
    // StructDecl(Rc<RefCell<r#struct::Decl>) // ?
}

impl InferType for Statement {
    fn infer_type(&self, _scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>> {
        None
    }
}
