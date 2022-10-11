use super::{BuiltinType, Expr, InferType};
use crate::ast;
use std::rc::Rc;

pub struct VarDecl {
    pub id: ast::Id,
    pub r#type: BuiltinType,
    pub expr: Rc<Expr>,
}

impl InferType for VarDecl {
    fn infer_type(&self) -> BuiltinType {
        BuiltinType::Void
    }
}
