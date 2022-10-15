use super::{Exportable, HasASTId};
use crate::{
    ast,
    dst::{self},
    location::HasSpan,
    program::Program,
    unit::Unit,
    Location, Panic,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub trait Scope {
    /// Return path to self.
    fn path(&self) -> PathBuf;

    /// Return the containing unit.
    fn unit(&self) -> Rc<RefCell<Unit>>;

    /// Search directly in the builtin scope.
    fn search_builtin(&self, id: &ast::Id) -> Option<Exportable>;

    /// Search in self.
    fn search(&self, id: &ast::Id) -> Option<Exportable>;

    fn push_decorator(&mut self, decorator: dst::decorator::Application);
    fn pop_decorators(&mut self) -> Vec<dst::decorator::Application>;

    /// Onyx-panic if `search` returns `Some`.
    fn ensure_not_found(&self, id: &ast::Id) -> Result<(), Panic> {
        if let Some(found) = self.search(id) {
            let mut panic = Panic::new(
                format!("`{}` already declared", id.value),
                Some(Location::new(self.unit(), id.span())),
            );

            panic.add_note(
                "Previously declared here".to_string(),
                Some(Location::new(self.unit(), found.ast_id().span())),
            );

            return Err(panic);
        }

        Ok(())
    }
}

impl Scope for dst::Mod {
    fn path(&self) -> PathBuf {
        self.unit.upgrade().unwrap().as_ref().borrow().path.clone()
    }

    fn unit(&self) -> Rc<RefCell<Unit>> {
        self.unit.upgrade().unwrap()
    }

    fn search_builtin(&self, id: &ast::Id) -> Option<Exportable> {
        println!("Searching builtin for `{}`", id);

        let self_unit = self.unit.upgrade().unwrap();

        let dependency = Program::resolve(
            self_unit.as_ref().borrow().program.upgrade().unwrap(),
            "builtin".into(),
        );

        if dependency.is_err() {
            panic!("Failed to resolve builtin");
        }

        dependency
            .unwrap()
            .as_ref()
            .borrow()
            .dst
            .as_ref()
            .unwrap()
            .search(id)
    }

    fn search(&self, id: &ast::Id) -> Option<Exportable> {
        println!("Searching \"{}\" for `{}`", self.path().display(), id);

        for i in self.imports.iter() {
            if i.0 == &id.value {
                println!("Found import for `{}`", id);
                return Some(i.1.import.clone());
            }
        }

        for e in self.exports.iter() {
            if e.0 == &id.value {
                println!("Found export for `{}`", id);
                return Some(e.1.clone());
            }
        }

        for i in self.declarations.iter() {
            if i.0 == &id.value {
                println!("Found declaration for `{}`", id);
                return Some(i.1.clone());
            }
        }

        if self.path() != PathBuf::from("builtin") && !self.path().starts_with("builtin/") {
            if let Some(found) = self.search_builtin(id) {
                println!("Found builtin for `{}`", id);
                return Some(found);
            }
        }

        println!("Not found `{}`", id);
        None
    }

    fn push_decorator(&mut self, decorator: dst::decorator::Application) {
        self.decorators_stack.push(decorator);
    }

    fn pop_decorators(&mut self) -> Vec<dst::decorator::Application> {
        let mut new = Vec::new();

        while let Some(decorator) = self.decorators_stack.pop() {
            new.push(decorator);
        }

        new
    }
}
