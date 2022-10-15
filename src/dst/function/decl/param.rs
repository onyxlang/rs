use std::{cell::RefCell, rc::Rc};

use crate::{
    ast,
    dst::{r#struct, HasId},
};

#[derive(Debug)]
pub struct Param {
    pub id: ast::Id,
    pub r#type: Rc<RefCell<r#struct::Decl>>,
}

impl Param {
    pub fn new(id: ast::Id, r#type: Rc<RefCell<r#struct::Decl>>) -> Self {
        Self { id, r#type }
    }
}

impl HasId for Param {
    fn id(&self) -> ast::Id {
        self.id.clone()
    }
}
