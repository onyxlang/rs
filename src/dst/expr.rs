use super::{Assignment, BuiltinType, InferType, MacroCall, VarRef};
use crate::{
    ast,
    location::{HasSpan, Span},
};

pub enum Expr {
    BoolLiteral(ast::Bool),
    VarRef(VarRef),
    MacroCall(MacroCall),
    Assignment(Assignment),
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::BoolLiteral(b) => b.span(),
            Expr::VarRef(v) => v.span(),
            Expr::MacroCall(m) => m.span(),
            Expr::Assignment(a) => a.span(),
        }
    }
}

impl InferType for Expr {
    fn infer_type(&self) -> BuiltinType {
        match self {
            Expr::BoolLiteral(_) => BuiltinType::Bool,
            Expr::VarRef(var) => var.infer_type(),
            Expr::MacroCall(m) => m.infer_type(),
            Expr::Assignment(a) => a.infer_type(),
        }
    }
}
