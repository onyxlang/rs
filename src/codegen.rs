use std::io::{self, Write};

use crate::dst;

pub trait Codegen {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()>;
}

impl Codegen for dst::Module {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()> {
        writeln!(w, "pub fn main() void {{")?;

        for stmt in self.main.iter() {
            stmt.codegen(w)?;
            writeln!(w)?;
        }

        writeln!(w, "}}")
    }
}

impl Codegen for dst::Statement {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            dst::Statement::VarDecl(var) => {
                var.codegen(w)?;
            }
            dst::Statement::TerminatedExpr(expr) => {
                expr.codegen(w)?;
            }
        }

        write!(w, ";")
    }
}

impl Codegen for dst::VarRef {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "@\"{}\"", self.decl.id.value)
    }
}

impl Codegen for dst::Expr {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            dst::Expr::BoolLiteral(b) => write!(w, "{}", b.value),
            dst::Expr::VarRef(var) => var.codegen(w),
            dst::Expr::MacroCall(m) => m.codegen(w),
            dst::Expr::Assignment(a) => {
                a.lhs.codegen(w)?;
                write!(w, " = ")?;
                a.rhs.codegen(w)
            }
        }
    }
}

impl Codegen for dst::VarDecl {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "var @\"{}\" = ", self.id.value)?;
        self.expr.codegen(w)?;
        Ok(())
    }
}

impl Codegen for dst::MacroCall {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            dst::MacroCall::Assert(_, expr) => {
                write!(w, "@import(\"std\").debug.assert(")?;
                expr.codegen(w)?;
                write!(w, ")")?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::resolve::Resolve;
    use crate::scope::Dummy;

    fn assert_codegen(input: &str, expected: &str) {
        let ast_module = crate::parser::parse("", input).expect("Failed to parse");
        let program = Dummy::default();
        let dst_module = ast_module.resolve(&program).expect("Failed to resolve");
        let mut buf = Vec::<u8>::new();
        dst_module.codegen(&mut buf).expect("Failed to codegen");
        assert_eq!(String::from_utf8(buf).unwrap(), expected);
    }

    #[test]
    pub fn test_assert() {
        assert_codegen(
            r#"
let a = true
@assert(a)
            "#,
            r#"pub fn main() void {
var @"a" = true;
@import("std").debug.assert(@"a");
}
"#,
        );
    }
    #[test]
    pub fn test_assignment() {
        assert_codegen(
            r#"
let a = false
a = true;
@assert(a)
            "#,
            r#"pub fn main() void {
var @"a" = false;
@"a" = true;
@import("std").debug.assert(@"a");
}
"#,
        );
    }
}
