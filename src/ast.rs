use std::fmt::{Debug, Formatter};

use crate::location::{HasSpan, Span};

/// A generic text node.
#[derive(Clone)]
pub struct Node {
    span: Span,
    pub text: String,
}

impl Node {
    pub fn new(span: Span, text: String) -> Self {
        Self { span, text }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl HasSpan for Node {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    BoolLiteral(Bool),
    IdRef(Id),
    MacroCall(MacroCall),
    Binop(Binop),
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::BoolLiteral(b) => b.span(),
            Expr::IdRef(id) => id.span(),
            Expr::MacroCall(m) => m.span(),
            Expr::Binop(b) => b.span(),
        }
    }
}

/// A boolean literal node.
#[derive(Clone)]
pub struct Bool {
    span: Span,
    pub value: bool,
}

impl Bool {
    pub fn new(span: Span, value: bool) -> Self {
        Self { span, value }
    }
}

impl PartialEq for Bool {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Debug for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl HasSpan for Bool {
    fn span(&self) -> Span {
        self.span
    }
}

/// An Onyx identifier node.
#[derive(Clone)]
pub struct Id {
    span: Span,
    pub value: String,
}

impl Id {
    pub fn new(span: Span, value: String) -> Self {
        Self { span, value }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl HasSpan for Id {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone)]
pub struct MacroCall {
    span: Span,
    pub id: Node,
    pub args: Vec<Expr>,
}

impl MacroCall {
    pub fn new(span: Span, id: Node, args: Vec<Expr>) -> Self {
        Self { span, id, args }
    }
}

impl PartialEq for MacroCall {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.args == other.args
    }
}

impl Debug for MacroCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.id, self.args)
    }
}

impl HasSpan for MacroCall {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone)]
pub struct Binop {
    span: Span,
    pub lhs: Box<Expr>,

    /// ADHOC: Can not have `Node` here due to `peg` limitations:
    /// a labeled capture is not supported within a `precedence!` macro.
    pub op: String,

    pub rhs: Box<Expr>,
}

impl Binop {
    pub fn new(lhs: Expr, op: String, rhs: Expr) -> Self {
        Self {
            span: lhs.span().join(rhs.span()),
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl PartialEq for Binop {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.op == other.op && self.rhs == other.rhs
    }
}

impl Debug for Binop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?}",
            self.lhs.as_ref(),
            self.op,
            self.rhs.as_ref()
        )
    }
}

impl HasSpan for Binop {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    VarDecl(VarDecl),
    TerminatedExpr(Expr),
}

impl HasSpan for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::VarDecl(v) => v.span(),
            Statement::TerminatedExpr(e) => e.span(),
        }
    }
}

pub struct VarDecl {
    span: Span,
    pub id: Id,
    pub expr: Expr,
}

impl VarDecl {
    pub fn new(span: Span, id: Id, expr: Expr) -> Self {
        Self { span, id, expr }
    }
}

impl PartialEq for VarDecl {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr
    }
}

impl Debug for VarDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = {:?}", self.id, self.expr)
    }
}

impl HasSpan for VarDecl {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(PartialEq, Debug)]
pub enum BlockBody {
    Comment(Comment),
    Statement(Statement),
    Expr(Expr),
}

impl HasSpan for BlockBody {
    fn span(&self) -> Span {
        match self {
            BlockBody::Comment(c) => c.span(),
            BlockBody::Statement(s) => s.span(),
            BlockBody::Expr(e) => e.span(),
        }
    }
}

pub struct Comment {
    span: Span,
    pub text: String,
}

impl Comment {
    pub fn new(span: Span, text: String) -> Self {
        Self { span, text }
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Debug for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "##{}", self.text)
    }
}

impl HasSpan for Comment {
    fn span(&self) -> Span {
        self.span
    }
}

/// An ASt module corresponds to a single source file.
#[derive(PartialEq, Debug)]
pub struct Module {
    pub body: Vec<BlockBody>,
}

impl Module {
    pub fn new(body: Vec<BlockBody>) -> Self {
        Self { body }
    }
}
