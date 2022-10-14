use crate::dst::{self, HasId};
use std::io::{self, Write};

pub trait Codegen {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()>;
}

impl Codegen for dst::Mod {
    fn codegen(&self, w: &mut dyn Write) -> io::Result<()> {
        for decl in self.declarations.iter() {
            match decl.1 {
                dst::Exportable::StructDecl(decl) => {
                    if let Some(builtin) = decl.as_ref().borrow().builtin {
                        match builtin {
                            dst::r#struct::Builtin::Bool => {
                                // Do not write anything, use `bool`.
                            }
                        }
                    } else {
                        unimplemented!("Lowering non-builtin structs")
                    }
                }
                dst::Exportable::VarDecl(_) => {
                    // Currently variables are only declared for main.
                }
            }
        }

        for import in self.imports.iter() {
            match &import.1.import {
                dst::Exportable::StructDecl(decl) => {
                    if let Some(builtin) = decl.as_ref().borrow().builtin {
                        match builtin {
                            dst::r#struct::Builtin::Bool => {
                                // Do not import anything, use `bool`.
                            }
                        }
                    } else {
                        unimplemented!("Lowering non-builtin structs")
                    }
                }
                dst::Exportable::VarDecl(_) => {
                    unimplemented!()
                }
            }
        }

        writeln!(w, "pub fn main() void {{")?;

        // TODO: Call `main` from dependencies.

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
        write!(w, "@\"{}\"", self.decl.id())
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
        write!(w, "var @\"{}\" = ", self.id())?;
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
    use std::rc::Weak;

    use super::*;

    fn assert_codegen(input: &str, expected: &str) {
        let ast_module = crate::parser::parse("".into(), input).expect("Failed to parse");
        let dst_module = ast_module.resolve(Weak::new()).expect("Failed to resolve");
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
