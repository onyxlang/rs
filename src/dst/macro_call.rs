use super::{BuiltinType, Expr, InferType};
use crate::{
    ast,
    location::{HasSpan, Span},
};
use std::rc::Rc;

/// For now, a macro call is lowered to a specific Zig code.
pub enum MacroCall {
    Assert(ast::MacroCall, Rc<Expr>),
}

impl HasSpan for MacroCall {
    fn span(&self) -> Span {
        match self {
            MacroCall::Assert(m, _) => m.span(),
        }
    }
}

impl InferType for MacroCall {
    fn infer_type(&self) -> BuiltinType {
        match self {
            MacroCall::Assert(..) => BuiltinType::Void,
        }
    }
}
