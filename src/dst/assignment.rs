use super::{BuiltinType, Expr, InferType, VarRef};
use crate::location::{HasSpan, Span};
use std::rc::Rc;

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
    fn infer_type(&self) -> BuiltinType {
        self.lhs.infer_type()
    }
}
