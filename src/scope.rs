use std::rc::Rc;

use crate::dst;

pub trait Scope {
    fn path(&self) -> String;
    fn find(&self, id: &str) -> Option<Rc<dst::VarDecl>>;
}

impl Scope for dst::Mod {
    fn path(&self) -> String {
        self.path.clone()
    }

    fn find(&self, id: &str) -> Option<Rc<dst::VarDecl>> {
        for var in self.var_decls.iter() {
            if var.id.value == id {
                return Some(Rc::clone(var));
            }
        }

        None
    }
}

#[derive(Default)]
pub struct Dummy {}

impl Scope for Dummy {
    fn path(&self) -> String {
        "dummy".to_string()
    }

    fn find(&self, _id: &str) -> Option<Rc<dst::VarDecl>> {
        None
    }
}
