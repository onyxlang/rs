use std::rc::Rc;

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

pub trait InferType {
    fn infer_type(&self) -> BuiltinType;
}

/// A DST module corresponds to a single source file.
pub struct Mod {
    pub path: String,
    pub var_decls: Vec<Rc<VarDecl>>,
    pub main: Vec<Statement>,
}

impl Mod {
    pub fn new(path: String) -> Self {
        Self {
            path,
            var_decls: Vec::new(),
            main: Vec::new(),
        }
    }
}
