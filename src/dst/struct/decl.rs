use super::{Builtin, Impl};
use crate::{
    ast,
    dst::{HasASTId, HasId},
};
use std::{fmt::Display, rc::Rc};

/// A struct declaration node.
#[derive(Debug)]
pub struct Decl {
    // TODO: May be either `decl` or `def` AST node.
    ast_node: ast::r#struct::Def,
    impls: Vec<Rc<Impl>>,
    pub builtin: Option<Builtin>,
}

impl Decl {
    pub fn new(ast_node: ast::r#struct::Def, builtin: Option<Builtin>) -> Self {
        Self {
            ast_node,
            impls: Vec::new(),
            builtin,
        }
    }

    pub fn add_impl(&mut self, r#impl: Rc<Impl>) {
        self.impls.push(r#impl);
    }
}

impl HasId for Decl {
    fn id(&self) -> String {
        self.ast_node.id.value.clone()
    }
}

impl HasASTId for Decl {
    fn ast_id(&self) -> ast::Id {
        self.ast_node.id.clone()
    }
}

impl PartialEq for Decl {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct {}", self.id())
    }
}
