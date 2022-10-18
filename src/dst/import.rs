use super::Exportable;
use crate::ast;

pub struct Import {
    #[allow(dead_code)]
    ast_node: ast::Import,

    pub imported: Vec<Exportable>,
}

impl Import {
    pub fn new(ast_node: ast::Import, entities: Vec<Exportable>) -> Self {
        Self {
            ast_node,
            imported: entities,
        }
    }
}
