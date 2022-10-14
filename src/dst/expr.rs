use super::{Assignment, BuiltinType, InferType, MacroCall, VarRef};
use crate::{
    ast::{self},
    location::{HasSpan, Span},
};

#[derive(Debug)]
pub enum Expr {
    BoolLiteral(ast::literal::Bool),
    VarRef(VarRef),
    MacroCall(MacroCall),
    Assignment(Assignment),
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::BoolLiteral(b) => b.span(),
            Expr::VarRef(r) => r.span(),
            Expr::MacroCall(m) => m.span(),
            Expr::Assignment(a) => a.span(),
        }
    }
}

impl InferType for Expr {
    fn infer_type(&self) -> BuiltinType {
        match self {
            Expr::BoolLiteral(_) => BuiltinType::Bool,
            Expr::VarRef(r) => r.infer_type(),
            Expr::MacroCall(m) => m.infer_type(),
            Expr::Assignment(a) => a.infer_type(),
        }
    }
}
