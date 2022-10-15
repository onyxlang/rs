use super::{function, r#struct, HasId, VarDecl};
use crate::ast;
use std::{cell::RefCell, rc::Rc};

// TODO: Replace with `Rc<dyn HasASTId>`?
#[derive(Clone, Debug)]
pub enum Exportable {
    VarDecl(Rc<VarDecl>),
    StructDecl(Rc<RefCell<r#struct::Decl>>),
    FunctionDecl(Rc<RefCell<function::Decl>>),
}

impl HasId for Exportable {
    fn id(&self) -> ast::Id {
        match self {
            Exportable::VarDecl(decl) => decl.id(),
            Exportable::StructDecl(decl) => decl.borrow().id(),
            Exportable::FunctionDecl(decl) => decl.borrow().id(),
        }
    }
}
