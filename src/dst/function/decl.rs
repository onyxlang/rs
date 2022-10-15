use super::Builtin;
use crate::{
    ast,
    dst::{r#struct, HasASTId, HasId},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Decl {
    ast_node: ast::function::Decl,
    pub builtin: Option<Builtin>,
    pub params: Vec<Param>,

    /// `None` means no returned value, i.e. `void`.
    pub return_type: Option<Rc<RefCell<r#struct::Decl>>>,
}

#[derive(Debug)]
pub struct Param {
    pub id: ast::Id,
    pub r#type: Rc<RefCell<r#struct::Decl>>,
}

impl Decl {
    pub fn new(
        ast_node: ast::function::Decl,
        builtin: Option<Builtin>,
        params: Vec<Param>,
        return_type: Option<Rc<RefCell<r#struct::Decl>>>,
    ) -> Self {
        Self {
            ast_node,
            builtin,
            params,
            return_type,
        }
    }
}

impl HasASTId for Decl {
    fn ast_id(&self) -> ast::Id {
        self.ast_node.id.clone()
    }
}

impl HasId for Decl {
    fn id(&self) -> String {
        self.ast_id().value
    }
}

impl Param {
    pub fn new(id: ast::Id, r#type: Rc<RefCell<r#struct::Decl>>) -> Self {
        Self { id, r#type }
    }
}

impl HasASTId for Param {
    fn ast_id(&self) -> ast::Id {
        self.id.clone()
    }
}

impl HasId for Param {
    fn id(&self) -> String {
        self.ast_id().value
    }
}
