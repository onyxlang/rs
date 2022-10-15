use std::{cell::RefCell, rc::Rc};

use crate::{ast, dst, location::HasSpan, Location, Panic};

use super::Resolve;

impl Resolve<Rc<RefCell<dst::r#struct::Decl>>> for ast::Qualifier {
    fn resolve(
        &self,
        scope: &mut dyn dst::Scope,
    ) -> Result<Rc<RefCell<dst::r#struct::Decl>>, Panic> {
        let found = scope.search(&self.id).ok_or_else(|| {
            Panic::new(
                format!("Undeclared {}", self),
                Some(Location::new(scope.unit(), self.span())),
            )
        })?;

        if let dst::Exportable::StructDecl(decl) = found {
            Ok(decl)
        } else {
            Err(Panic::new(
                format!("{} is not a struct", self),
                Some(Location::new(scope.unit(), self.span())),
            ))
        }
    }
}

impl Resolve<Rc<RefCell<dst::function::Decl>>> for ast::Qualifier {
    fn resolve(
        &self,
        scope: &mut dyn dst::Scope,
    ) -> Result<Rc<RefCell<dst::function::Decl>>, Panic> {
        let found = scope.search(&self.id).ok_or_else(|| {
            Panic::new(
                format!("Undeclared {}", self),
                Some(Location::new(scope.unit(), self.span())),
            )
        })?;

        if let dst::Exportable::FunctionDecl(decl) = found {
            Ok(decl)
        } else {
            Err(Panic::new(
                format!("{} is not a function", self),
                Some(Location::new(scope.unit(), self.span())),
            ))
        }
    }
}
