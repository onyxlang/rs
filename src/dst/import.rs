use super::Exportable;
use crate::ast;

pub struct Import {
    ast_node: ast::Import,
    pub import: Exportable,
}

impl Import {
    pub fn new(ast_node: ast::Import, import: Exportable) -> Self {
        Self { ast_node, import }
    }

    pub fn id(&self) -> &ast::Id {
        &self.ast_node.id
    }
}
