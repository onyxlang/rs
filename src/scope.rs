use std::rc::Rc;

use crate::dst;

pub trait Scope {
    fn find(&self, id: &str) -> Option<Rc<dst::VarDecl>>;
}

impl Scope for dst::Module {
    fn find(&self, id: &str) -> Option<Rc<dst::VarDecl>> {
        for var in self.var_decls.iter() {
            if var.id == id {
                return Some(Rc::clone(var));
            }
        }

        None
    }
}

#[derive(Default)]
pub struct Dummy {}

impl Scope for Dummy {
    fn find(&self, _id: &str) -> Option<Rc<dst::VarDecl>> {
        None
    }
}
