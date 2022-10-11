#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    BoolLiteral(bool),
    IdRef(String),
    MacroCall(MacroCall),
    Binop(Binop),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Binop {
    pub lhs: Box<Expr>,
    pub op: String,
    pub rhs: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    VarDecl(VarDecl),
    TerminatedExpr(Expr),
}

#[derive(PartialEq, Eq, Debug)]
pub struct MacroCall {
    pub id: String,
    pub args: Vec<Expr>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct VarDecl {
    pub id: String,
    pub expr: Expr,
}

#[derive(PartialEq, Eq, Debug)]
pub enum BlockBody {
    Comment(Comment),
    Statement(Statement),
    Expr(Expr),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Comment {
    pub text: String,
}

/// An ASt module corresponds to a single source file.
#[derive(PartialEq, Eq, Debug)]
pub struct Module {
    pub body: Vec<BlockBody>,
}

impl Module {
    pub fn new(body: Vec<BlockBody>) -> Self {
        Self { body }
    }
}
