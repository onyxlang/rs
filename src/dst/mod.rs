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
    pub imports: std::collections::HashMap<String, Exportable>,

    pub decorators_stack: Vec<decorator::Application>,
    pub declarations: std::collections::HashMap<String, Exportable>,
}

impl Mod {
    pub fn new(unit: Weak<RefCell<Unit>>) -> Self {
        Self {
            unit,
            main: Vec::new(),
            exports: std::collections::HashMap::new(),
            imports: std::collections::HashMap::new(),
            decorators_stack: Vec::new(),
            declarations: std::collections::HashMap::new(),
        }
    }

    pub fn resolve_dependency(
        &mut self,
        from: ast::literal::String,
    ) -> Result<Rc<RefCell<Unit>>, Panic> {
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

        // Find in the self_unit dependencies if it's already there.
        let dependency = self_unit
            .as_ref()
            .borrow_mut()
            .dependencies
            .get(&path)
            .cloned();

        if let Some(dependency) = dependency {
            Ok(dependency.upgrade().unwrap())
        } else {
            let strong = Program::resolve(
                self_unit.as_ref().borrow().program.upgrade().unwrap(),
                path.clone(),
            )?;

            let weak = Rc::downgrade(&strong);

            self_unit
                .as_ref()
                .borrow_mut()
                .dependencies
                .insert(path, weak); // TODO: Expect `None` here.

            Ok(strong)
        }
    }

    pub fn store(&mut self, entity: Exportable) -> Result<(), Panic> {
        self.ensure_not_found(&entity.id())?;
        self.declarations.insert(entity.id().value, entity);
        Ok(())
    }
}
