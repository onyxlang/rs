use super::{r#struct, HasId, Mod, VarDecl};
use crate::{
    ast,
    dst::{self},
    location::HasSpan,
    program::Program,
    Location, Panic,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

#[derive(Clone, Debug)]
pub enum Exportable {
    VarDecl(Rc<VarDecl>),
    StructDecl(Rc<RefCell<r#struct::Decl>>),
}

impl HasId for Exportable {
    fn id(&self) -> String {
        match self {
            Exportable::VarDecl(decl) => decl.id(),
            Exportable::StructDecl(decl) => decl.borrow().id(),
        }
    }
}

pub trait Scope {
    fn path(&self) -> PathBuf;

    /// Search in self.
    fn search(&self, id: &str) -> Option<Exportable>;

    /// NOTE: Shall call `add_import` after resolving.
    fn resolve_import(&mut self, node: ast::Import) -> Result<dst::Import, Panic>;

    fn add_import(&mut self, id: &str, import: dst::Import);

    fn push_decorator(&mut self, decorator: dst::decorator::Application);
    fn pop_decorators(&mut self) -> Vec<dst::decorator::Application>;

    fn store(&mut self, entity: Exportable);
}

impl Scope for dst::Mod {
    fn path(&self) -> PathBuf {
        self.unit.upgrade().unwrap().as_ref().borrow().path.clone()
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

    fn add_import(&mut self, id: &str, import: dst::Import) {
        self.imports.insert(id.to_string(), import);
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

    fn store(&mut self, entity: Exportable) {
        self.declarations.insert(entity.id(), entity);
    }
}

impl Mod {
    pub fn add_export(&mut self, id: &str, export: Exportable) {
        self.exports.insert(id.to_string(), export);
    }
}
