use super::Exportable;
use crate::ast;

pub struct Export {
    ast_node: ast::Export,
    pub export: Exportable,
}

impl Export {
    pub fn new(ast_node: ast::Export, export: Exportable) -> Self {
        Self { ast_node, export }
    }

    pub fn id(&self) -> &ast::Id {
        &self.ast_node.id
    }
}
