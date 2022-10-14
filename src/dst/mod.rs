use std::{cell::RefCell, rc::Weak};

mod scope;
pub use scope::Exportable;
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

use crate::{ast, unit::Unit};

pub trait InferType {
    fn infer_type(&self) -> BuiltinType;
}

pub trait HasId {
    fn id(&self) -> String;
}

pub trait HasASTId {
    fn ast_id(&self) -> ast::Id;
}

/// A DST module corresponds to a single source file.
pub struct Mod {
    pub unit: Weak<RefCell<Unit>>,
    pub main: Vec<Statement>,

    pub exports: std::collections::HashMap<String, Exportable>,
    pub default: Option<Exportable>,
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
            default: None,
            imports: std::collections::HashMap::new(),
            decorators_stack: Vec::new(),
            declarations: std::collections::HashMap::new(),
        }
    }
}
