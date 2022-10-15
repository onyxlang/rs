use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

mod scope;
pub use scope::Scope;

mod var_decl;
pub use var_decl::VarDecl;

mod builtin_type;
pub use builtin_type::BuiltinType;

mod expr;
pub use expr::Expr;

mod var_ref;
pub use var_ref::VarRef;

mod assignment;
pub use assignment::Assignment;

mod macro_call;
pub use macro_call::MacroCall;

mod statement;
pub use statement::Statement;

pub mod decorator;
pub mod r#struct;

pub mod import;
pub use import::Import;

mod export;
pub use export::Export;

pub mod function;

mod exportable;
pub use exportable::Exportable;

mod call;
pub use call::Call;

use crate::{ast, location::HasSpan, program::Program, unit::Unit, Location, Panic};

pub trait InferType {
    fn infer_type(&self, scope: &dyn Scope) -> Option<Rc<RefCell<r#struct::Decl>>>;
}

pub trait HasId {
    fn id(&self) -> ast::Id;
}

pub trait HasQualifier {
    fn qualifier(&self) -> ast::Qualifier;
}

/// A DST module corresponds to a single source file.
pub struct Mod {
    pub unit: Weak<RefCell<Unit>>,
    pub main: Vec<Statement>,

    pub exports: std::collections::HashMap<String, Exportable>,
    pub default_export: Option<Exportable>,
    pub imports: std::collections::HashMap<String, Import>,

    pub decorators_stack: Vec<decorator::Application>,
    pub declarations: std::collections::HashMap<String, Exportable>,
}

impl Mod {
    pub fn new(unit: Weak<RefCell<Unit>>) -> Self {
        Self {
            unit,
            main: Vec::new(),
            exports: std::collections::HashMap::new(),
            default_export: None,
            imports: std::collections::HashMap::new(),
            decorators_stack: Vec::new(),
            declarations: std::collections::HashMap::new(),
        }
    }

    pub fn add_export(&mut self, id: &ast::Id, export: Exportable) -> Result<(), Panic> {
        self.ensure_not_found(id)?;
        self.exports.insert(id.value.clone(), export);
        Ok(())
    }

    pub fn add_import(&mut self, id: &ast::Id, import: Import) -> Result<(), Panic> {
        self.ensure_not_found(id)?;
        self.imports.insert(id.value.clone(), import);
        Ok(())
    }

    // TODO: Check if the id is already declared.
    pub fn resolve_default_import(
        &mut self,
        from: ast::literal::String,
    ) -> Result<Exportable, Panic> {
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

    pub fn store(&mut self, entity: Exportable) -> Result<(), Panic> {
        self.ensure_not_found(&entity.id())?;
        self.declarations.insert(entity.id().value, entity);
        Ok(())
    }
}
