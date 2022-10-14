use super::{BuiltinType, Expr, HasASTId, HasId, InferType};
use crate::{
    ast,
    location::{HasSpan, Span},
};
use std::rc::Rc;

#[derive(Debug)]
pub struct VarDecl {
    ast_node: ast::VarDecl,
    pub r#type: BuiltinType,
    pub expr: Rc<Expr>,
}

impl VarDecl {
    pub fn new(ast_node: ast::VarDecl, r#type: BuiltinType, expr: Rc<Expr>) -> Self {
        Self {
            ast_node,
            r#type,
            expr,
        }
    }
}

impl HasASTId for VarDecl {
    fn ast_id(&self) -> ast::Id {
        self.ast_node.id.clone()
    }
}

impl HasId for VarDecl {
    fn id(&self) -> String {
        self.ast_id().value
    }
}

impl HasSpan for VarDecl {
    fn span(&self) -> Span {
        self.ast_node.span()
    }
}

impl InferType for VarDecl {
    fn infer_type(&self) -> BuiltinType {
        BuiltinType::Void
    }
}
