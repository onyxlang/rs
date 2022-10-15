mod expr;
pub use expr::Expr;

mod id;
pub use id::Id;

mod macro_call;
pub use macro_call::MacroCall;

mod binop;
pub use binop::Binop;

mod statement;
pub use statement::Statement;

mod var_decl;
pub use var_decl::VarDecl;

mod block_body;
pub use block_body::BlockBody;

mod comment;
pub use comment::Comment;

pub mod literal;

mod import;
pub use import::Import;

mod decorator;
pub use decorator::Decorator;

pub mod function;
pub mod r#struct;

mod call;
pub use call::Call;

use std::fmt::Debug;

/// An ASt module corresponds to a single source file.
#[derive(PartialEq, Debug)]
pub struct Mod {
    pub body: Vec<BlockBody>,
}

impl Mod {
    pub fn new(body: Vec<BlockBody>) -> Self {
        Self { body }
    }
}
