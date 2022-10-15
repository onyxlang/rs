use super::{function, r#struct, Expr, InferType, Scope};
use crate::{ast, location::HasSpan};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Call {
    pub ast_node: ast::Call,
    pub callee: Rc<RefCell<function::Decl>>,
    pub args: Vec<Rc<Expr>>,
}

impl Call {
    pub fn new(
        ast_node: ast::Call,
        callee: Rc<RefCell<function::Decl>>,
        args: Vec<Rc<Expr>>,
    ) -> Self {
        Self {
            ast_node,
            callee,
            args,
        }
    }
}

impl InferType for Call {
    fn infer_type(&self, _scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>> {
        self.callee.borrow().return_type.clone()
    }
}

impl HasSpan for Call {
    fn span(&self) -> crate::location::Span {
        self.ast_node.span()
    }
}
