use super::{BuiltinType, InferType, VarDecl};
use crate::location::{HasSpan, Span};
use std::rc::Rc;

#[derive(Clone)]
pub struct VarRef {
    pub decl: Rc<VarDecl>,
}

impl HasSpan for VarRef {
    fn span(&self) -> Span {
        self.decl.id.span()
    }
}

impl InferType for VarRef {
    fn infer_type(&self) -> BuiltinType {
        self.decl.r#type
    }
}
