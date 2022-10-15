use super::{Exportable, HasASTId, HasId, Mod};
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
    fn path(&self) -> PathBuf;
    fn unit(&self) -> Rc<RefCell<Unit>>;

    fn search_builtin(&self, id: &str) -> Option<Exportable>;

    /// Search in self.
    fn search(&self, id: &str) -> Option<Exportable>;

    /// NOTE: Shall call `add_import` after resolving.
    fn resolve_default_import(&mut self, from: ast::literal::String) -> Result<Exportable, Panic>;

    /// Should call `ensure_not_stored` within.
    fn add_import(&mut self, id: &ast::Id, import: dst::Import) -> Result<(), Panic>;

    fn push_decorator(&mut self, decorator: dst::decorator::Application);
    fn pop_decorators(&mut self) -> Vec<dst::decorator::Application>;

    /// Should call `ensure_not_stored` within.
    fn store(&mut self, entity: Exportable) -> Result<(), Panic>;

    /// May panic if already has a declaration with the same id.
    fn ensure_not_stored(&self, id: &ast::Id) -> Result<(), Panic> {
        if let Some(found) = self.search(&id.value) {
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

    fn search_builtin(&self, id: &str) -> Option<Exportable> {
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

    fn search(&self, id: &str) -> Option<Exportable> {
        println!("Searching \"{}\" for `{}`", self.path().display(), id);

        for i in self.imports.iter() {
            if i.0 == id {
                println!("Found import for `{}`", id);
                return Some(i.1.import.clone());
            }
        }

        for e in self.exports.iter() {
            if e.0 == id {
                println!("Found export for `{}`", id);
                return Some(e.1.clone());
            }
        }

        for i in self.declarations.iter() {
            if i.0 == id {
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

    fn add_import(&mut self, id: &ast::Id, import: dst::Import) -> Result<(), Panic> {
        self.ensure_not_stored(id)?;
        self.imports.insert(id.value.clone(), import);
        Ok(())
    }

    // TODO: Check if the id is already declared.
    fn resolve_default_import(&mut self, from: ast::literal::String) -> Result<Exportable, Panic> {
        let mut path = self.path();
        path.pop();
        path.push(from.value.clone());

        if path == self.path() {
            return Err(Panic::new(
                "Cannot import from self".to_string(),
                Some(Location::new(self.unit(), from.span())),
            ));
        }

        let self_unit = self.unit.upgrade().unwrap();

        let dependency = Program::resolve(
            self_unit.as_ref().borrow().program.upgrade().unwrap(),
            path.clone(),
        )?;

        let default = dependency
            .as_ref()
            .borrow()
            .dst
            .as_ref()
            .unwrap()
            .default_export
            .clone();

        if default.is_none() {
            return Err(Panic::new(
                format!(
                    "Module at \"{}\" doesn't have a default export",
                    path.display()
                ),
                Some(Location::new(self.unit(), from.span())),
            ));
        }

        self_unit
            .as_ref()
            .borrow_mut()
            .dependencies
            .push(Rc::downgrade(&dependency));

        Ok(default.unwrap())
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

    fn store(&mut self, entity: Exportable) -> Result<(), Panic> {
        self.ensure_not_stored(&entity.ast_id())?;
        self.declarations.insert(entity.id(), entity);
        Ok(())
    }
}

impl Mod {
    pub fn add_export(&mut self, id: &ast::Id, export: Exportable) -> Result<(), Panic> {
        self.ensure_not_stored(id)?;
        self.exports.insert(id.value.clone(), export);
        Ok(())
    }
}
