use super::{Exportable, HasASTId, HasId, Mod};
use crate::{
    ast,
    dst::{self},
    location::HasSpan,
    program::Program,
    Location, Panic,
};
use std::{path::PathBuf, rc::Rc};

pub trait Scope {
    fn path(&self) -> PathBuf;

    fn search_builtin(&self, id: &str) -> Option<Exportable>;

    /// Search in self.
    fn search(&self, id: &str) -> Option<Exportable>;

    /// NOTE: Shall call `add_import` after resolving.
    fn resolve_import(&mut self, node: ast::Import) -> Result<dst::Import, Panic>;

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
                Some(Location::new(self.path(), id.span())),
            );

            panic.add_note(
                "Previously declared here".to_string(),
                Some(Location::new(self.path(), found.ast_id().span())),
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

    fn search_builtin(&self, id: &str) -> Option<Exportable> {
        todo!()
    }

    fn search(&self, id: &str) -> Option<Exportable> {
        for i in self.imports.iter() {
            if i.0 == id {
                return Some(i.1.import.clone());
            }
        }

        for i in self.declarations.iter() {
            if i.0 == id {
                return Some(i.1.clone());
            }
        }

        None
    }

    fn add_import(&mut self, id: &ast::Id, import: dst::Import) -> Result<(), Panic> {
        self.ensure_not_stored(id)?;
        self.imports.insert(id.value.clone(), import);
        Ok(())
    }

    fn resolve_import(&mut self, node: ast::Import) -> Result<dst::Import, Panic> {
        let mut path = self.path();
        path.pop();
        path.push(node.from.value.clone());

        if path == self.path() {
            return Err(Panic::new(
                "Cannot import from self".to_string(),
                Some(Location::new(self.path(), node.from.span())),
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
            .default
            .clone();

        if default.is_none() {
            return Err(Panic::new(
                format!(
                    "Module at \"{}\" doesn't have a default export",
                    path.display()
                ),
                Some(Location::new(self.path(), node.from.span())),
            ));
        }

        let dst = dst::Import::new(node, default.unwrap());

        self_unit
            .as_ref()
            .borrow_mut()
            .dependencies
            .push(Rc::downgrade(&dependency));

        Ok(dst)
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
