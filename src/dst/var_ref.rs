use super::{r#struct, InferType, Scope, VarDecl};
use crate::{
    ast,
    location::{HasSpan, Span},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub struct VarRef {
    ast_node: ast::Id,
    pub decl: Rc<VarDecl>,
}

impl VarRef {
    pub fn new(ast_node: ast::Id, decl: Rc<VarDecl>) -> Self {
        Self { ast_node, decl }
    }
}

impl HasSpan for VarRef {
    fn span(&self) -> Span {
        self.ast_node.span()
    }
}

impl InferType for VarRef {
    fn infer_type(&self, _scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>> {
        Some(self.decl.r#type.clone())
    }
}
