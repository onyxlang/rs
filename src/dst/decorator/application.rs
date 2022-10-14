use super::Builtin;
use crate::ast;

pub struct Application {
    pub ast_node: ast::Decorator,
    pub decorator: Builtin,
}

impl Application {
    pub fn new(ast_node: ast::Decorator, resolved: Builtin) -> Self {
        Self {
            ast_node,
            decorator: resolved,
        }
    }
}
