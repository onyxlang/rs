use crate::dst::{self, HasId};
use std::io::{self, Write};

pub trait Lowerable {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()>;
}

impl Lowerable for dst::Mod {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()> {
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
                dst::Exportable::FunctionDecl(_) => {
                    // Function declarations aren't lowered.
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
                dst::Exportable::FunctionDecl(decl) => {
                    if decl.as_ref().borrow().builtin.is_some() {
                        // Do not import builtin function declarations.
                    } else {
                        unimplemented!("Lowering non-builtin structs")
                    }
                }
            }
        }

        writeln!(w, "pub fn main() void {{")?;

        // TODO: Call `main` from dependencies.

        for stmt in self.main.iter() {
            stmt.lower(w)?;
            writeln!(w)?;
        }

        writeln!(w, "}}")
    }
}

impl Lowerable for dst::Statement {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            dst::Statement::VarDecl(var) => {
                var.lower(w)?;
            }
            dst::Statement::TerminatedExpr(expr) => {
                expr.lower(w)?;
            }
        }

        write!(w, ";")
    }
}

impl Lowerable for dst::VarRef {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "@\"{}\"", self.decl.id())
    }
}

impl Lowerable for dst::Expr {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            dst::Expr::BoolLiteral(b) => write!(w, "{}", b.value),
            dst::Expr::VarRef(var) => var.lower(w),
            dst::Expr::MacroCall(m) => m.lower(w),
            dst::Expr::Assignment(a) => {
                a.lhs.lower(w)?;
                write!(w, " = ")?;
                a.rhs.lower(w)
            }
            dst::Expr::FunctionCall(c) => c.lower(w),
        }
    }
}

impl Lowerable for dst::Call {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(builtin) = &self.callee.as_ref().borrow().builtin {
            match builtin {
                dst::function::Builtin::BoolEq => {
                    self.args[0].lower(w)?;
                    write!(w, " == ")?;
                    self.args[1].lower(w)
                }
            }
        } else {
            unimplemented!("Lowering non-builtin structs")

            // write!(w, "{}(", self.callee.as_ref().borrow().id())?;

            // for (i, arg) in self.args.iter().enumerate() {
            //     if i > 0 {
            //         write!(w, ", ")?;
            //     }

            //     arg.lower(w)?;
            // }

            // write!(w, ")")
        }
    }
}

impl Lowerable for dst::VarDecl {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "var @\"{}\" = ", self.id())?;
        self.expr.lower(w)?;
        Ok(())
    }
}

impl Lowerable for dst::MacroCall {
    fn lower(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            dst::MacroCall::Assert(_, expr) => {
                write!(w, "@import(\"std\").debug.assert(")?;
                expr.lower(w)?;
                write!(w, ")")?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{program::Program, unit::Unit};
    use std::rc::Rc;

    // FIXME: Properly display panics (with source).
    fn assert_lowering(input: &str, expected: &str) {
        let ast_module = crate::parser::parse_simple(input);
        let program = Program::new(".cache".into());
        let unit = Unit::new(Rc::downgrade(&program), "<test>".into());
        let dst_module = ast_module
            .resolve(Rc::downgrade(&unit))
            .expect("Failed to resolve");
        let mut buf = Vec::<u8>::new();
        dst_module.lower(&mut buf).expect("Failed to lower");
        assert_eq!(String::from_utf8(buf).unwrap(), expected);
    }

    #[test]
    pub fn test_assert() {
        assert_lowering(
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
        assert_lowering(
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

    #[test]
    pub fn test_bool_eq() {
        assert_lowering(
            r#"
@[Builtin] decl function eq?(self: Bool, another: Bool) -> Bool
let a = false
let b = true
eq?(a, b);"#,
            r#"pub fn main() void {
var @"a" = false;
var @"b" = true;
@"a" == @"b";
}
"#,
        );
    }
}
