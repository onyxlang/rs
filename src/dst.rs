use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

use crate::{
    ast,
    location::{HasSpan, Span},
    unit::Unit,
};

pub struct VarDecl {
    pub id: ast::Id,
    pub r#type: BuiltinType,
    pub expr: Rc<Expr>,
}

impl InferType for VarDecl {
    fn infer_type(&self) -> BuiltinType {
        BuiltinType::Void
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BuiltinType {
    Void,
    Bool,
}

impl Display for BuiltinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinType::Void => write!(f, "Void"),
            BuiltinType::Bool => write!(f, "Bool"),
        }
    }
}

pub enum Expr {
    BoolLiteral(ast::Bool),
    VarRef(VarRef),
    MacroCall(MacroCall),
    Assignment(Assignment),
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::BoolLiteral(b) => b.span(),
            Expr::VarRef(v) => v.span(),
            Expr::MacroCall(m) => m.span(),
            Expr::Assignment(a) => a.span(),
        }
    }
}

#[derive(Clone)]
pub struct VarRef {
    pub decl: Rc<VarDecl>,
}

impl HasSpan for VarRef {
    fn span(&self) -> Span {
        self.decl.id.span()
    }
}

impl InferType for VarRef {
    fn infer_type(&self) -> BuiltinType {
        self.decl.r#type
    }
}

pub struct Assignment {
    pub lhs: VarRef,
    pub rhs: Rc<Expr>,
}

impl HasSpan for Assignment {
    fn span(&self) -> Span {
        self.lhs.span().join(self.rhs.span())
    }
}

impl InferType for Assignment {
    fn infer_type(&self) -> BuiltinType {
        self.lhs.infer_type()
    }
}

pub trait InferType {
    fn infer_type(&self) -> BuiltinType;
}

impl InferType for Expr {
    fn infer_type(&self) -> BuiltinType {
        match self {
            Expr::BoolLiteral(_) => BuiltinType::Bool,
            Expr::VarRef(var) => var.infer_type(),
            Expr::MacroCall(m) => m.infer_type(),
            Expr::Assignment(a) => a.infer_type(),
        }
    }
}

/// For now, a macro call is lowered to a specific Zig code.
pub enum MacroCall {
    Assert(ast::MacroCall, Rc<Expr>),
}

impl HasSpan for MacroCall {
    fn span(&self) -> Span {
        match self {
            MacroCall::Assert(m, _) => m.span(),
        }
    }
}

impl InferType for MacroCall {
    fn infer_type(&self) -> BuiltinType {
        match self {
            MacroCall::Assert(..) => BuiltinType::Void,
        }
    }
}

pub enum Statement {
    VarDecl(Rc<VarDecl>),
    TerminatedExpr(Rc<Expr>),
}

impl InferType for Statement {
    fn infer_type(&self) -> BuiltinType {
        BuiltinType::Void
    }
}

/// A DST module corresponds to a single source file.
pub struct Module {
    pub path: String,
    pub var_decls: Vec<Rc<VarDecl>>,
    pub main: Vec<Statement>,
}

impl Module {
    pub fn new(path: String) -> Self {
        Self {
            path,
            var_decls: Vec::new(),
            main: Vec::new(),
        }
    }
}
