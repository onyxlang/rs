use super::Builtin;
use crate::{
    ast,
    dst::{r#struct, HasId, HasQualifier},
};
use std::{cell::RefCell, rc::Rc};

mod param;
pub use param::Param;

#[derive(Debug)]
pub struct Decl {
    ast_node: ast::function::Decl,
    pub builtin: Option<Builtin>,
    pub params: Vec<Param>,

    /// `None` means no returned value, i.e. `void`.
    pub return_type: Option<Rc<RefCell<r#struct::Decl>>>,
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

impl HasId for Decl {
    fn id(&self) -> ast::Id {
        self.ast_node.id.id.clone()
    }
}

impl HasQualifier for Decl {
    fn qualifier(&self) -> ast::Qualifier {
        self.ast_node.id.clone()
    }
}
