use super::{r#struct, Expr, HasId, InferType, Scope};
use crate::{
    ast,
    location::{HasSpan, Span},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct VarDecl {
    ast_node: ast::VarDecl,
    pub r#type: Rc<RefCell<r#struct::Decl>>,
    pub expr: Rc<Expr>,
}

impl VarDecl {
    pub fn new(
        ast_node: ast::VarDecl,
        r#type: Rc<RefCell<r#struct::Decl>>,
        expr: Rc<Expr>,
    ) -> Self {
        Self {
            ast_node,
            r#type,
            expr,
        }
    }
}

impl HasId for VarDecl {
    fn id(&self) -> ast::Id {
        self.ast_node.id.clone()
    }
}

impl HasSpan for VarDecl {
    fn span(&self) -> Span {
        self.ast_node.span()
    }
}

impl InferType for VarDecl {
    fn infer_type(&self, _scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>> {
        None
    }
}
