use super::{r#struct, Expr, InferType, Scope, VarRef};
use crate::location::{HasSpan, Span};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Assignment {
    pub lhs: VarRef,
    pub rhs: Rc<Expr>,
}

impl HasSpan for Assignment {
    fn span(&self) -> Span {
        self.lhs.span().join(self.rhs.span())
    }
}

impl InferType for Assignment {
    fn infer_type(&self, scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>> {
        self.lhs.infer_type(scope)
    }
}
