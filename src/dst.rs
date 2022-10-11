use std::{fmt::Display, rc::Rc};

pub struct VarDecl {
    pub id: String,
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
    BoolLiteral(bool),
    VarRef(VarRef),
    MacroCall(MacroCall),
    Assignment(Assignment),
}

#[derive(Clone)]
pub struct VarRef {
    pub decl: Rc<VarDecl>,
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
    Assert(Rc<Expr>),
}

impl InferType for MacroCall {
    fn infer_type(&self) -> BuiltinType {
        match self {
            MacroCall::Assert(_) => BuiltinType::Void,
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
#[derive(Default)]
pub struct Module {
    pub var_decls: Vec<Rc<VarDecl>>,
    pub main: Vec<Statement>,
}
