use super::{r#struct, Expr, InferType, Scope};
use crate::{
    ast,
    location::{HasSpan, Span},
};
use std::{cell::RefCell, rc::Rc};

/// For now, a macro call is lowered to a specific Zig code.
#[derive(Debug)]
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
    fn infer_type(&self, _scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>> {
        match self {
            MacroCall::Assert(..) => None,
        }
    }
}
