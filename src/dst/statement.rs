use super::{BuiltinType, Expr, InferType, VarDecl};
use std::rc::Rc;

pub enum Statement {
    VarDecl(Rc<VarDecl>),
    TerminatedExpr(Rc<Expr>),
}

impl InferType for Statement {
    fn infer_type(&self) -> BuiltinType {
        BuiltinType::Void
    }
}