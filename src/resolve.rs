use std::rc::Rc;

use crate::ast;
use crate::dst;
use crate::dst::InferType;
use crate::location::HasSpan;
use crate::panic::Panic;
use crate::scope::Scope;
use crate::Location;

pub trait Resolve<T> {
    fn resolve(&self, scope: &dyn Scope) -> Result<T, Panic>;
}

impl Resolve<dst::Mod> for ast::Mod {
    fn resolve(&self, scope: &dyn Scope) -> Result<dst::Mod, Panic> {
        let mut dst_module = dst::Mod::new(scope.path());

        for body in &self.body {
            match body {
                ast::BlockBody::Stmt(stmt) => match stmt {
                    ast::Statement::VarDecl(var_decl) => {
                        let var = var_decl.resolve(&dst_module)?;
                        dst_module.var_decls.push(Rc::clone(&var));
                        dst_module.main.push(dst::Statement::VarDecl(var));
                    }
                    ast::Statement::TerminatedExpr(expr) => {
                        let expr = expr.resolve(&dst_module)?;
                        dst_module.main.push(dst::Statement::TerminatedExpr(expr));
                    }
                },
                ast::BlockBody::Expr(expr) => {
                    let expr = expr.resolve(&dst_module)?;

                    if expr.infer_type() != dst::BuiltinType::Void {
                        return Err(Panic::new(
                            "Unused expression result".to_string(),
                            Location::new(scope.path(), expr.span()),
                        ));
                    }

                    dst_module.main.push(dst::Statement::TerminatedExpr(expr));
                }
                ast::BlockBody::Comment(_) => {
                    // Do nothing.
                }
            }
        }

        Ok(dst_module)
    }
}

impl Resolve<Rc<dst::Expr>> for ast::Expr {
    fn resolve(&self, scope: &dyn Scope) -> Result<Rc<dst::Expr>, Panic> {
        match self {
            ast::Expr::BoolLiteral(b) => Ok(Rc::new(dst::Expr::BoolLiteral(b.clone()))),
            ast::Expr::IdRef(id) => {
                if let Some(var) = scope.find(id.value.as_str()) {
                    Ok(Rc::new(dst::Expr::VarRef(dst::VarRef {
                        decl: Rc::clone(&var),
                    })))
                } else {
                    Err(Panic::new(
                        format!("Unknown identifier: {}", id.value),
                        Location::new(scope.path(), id.span()),
                    ))
                }
            }
            ast::Expr::MacroCall(m) => Ok(Rc::new(dst::Expr::MacroCall(m.resolve(scope)?))),
            ast::Expr::Binop(b) => match b.op.as_str() {
                "=" => {
                    let lhs = b.lhs.resolve(scope)?;
                    let rhs = b.rhs.resolve(scope)?;

                    if let dst::Expr::VarRef(var) = &*lhs {
                        if var.decl.r#type != rhs.infer_type() {
                            return Err(Panic::new(
                                format!(
                                    "Type mismatch: expected {}, got {}",
                                    var.decl.r#type,
                                    rhs.infer_type()
                                ),
                                Location::new(scope.path(), rhs.span()),
                            ));
                        }

                        Ok(Rc::new(dst::Expr::Assignment(dst::Assignment {
                            lhs: var.clone(),
                            rhs,
                        })))
                    } else {
                        Err(Panic::new(
                            "Left-hand side of assignment must be a variable".to_string(),
                            Location::new(scope.path(), lhs.span()),
                        ))
                    }
                }
                &_ => todo!(),
            },
        }
    }
}

impl Resolve<Rc<dst::VarDecl>> for ast::VarDecl {
    /// Pushes the resolved variable declaration to the scope.
    fn resolve(&self, scope: &dyn Scope) -> Result<Rc<dst::VarDecl>, Panic> {
        let expr = self.expr.resolve(scope)?;
        let r#type = expr.infer_type();
        let var = Rc::new(dst::VarDecl {
            id: self.id.clone(),
            r#type,
            expr,
        });
        Ok(var)
    }
}

impl Resolve<dst::MacroCall> for ast::MacroCall {
    fn resolve(&self, scope: &dyn Scope) -> Result<dst::MacroCall, Panic> {
        match self.id.text.as_str() {
            "assert" => {
                assert_eq!(self.args.len(), 1);
                let arg = &self.args[0].resolve(scope)?;
                Ok(dst::MacroCall::Assert(self.clone(), Rc::clone(arg)))
            }
            _ => Err(Panic::new(
                format!("Unknown macro: {}", self.id.text),
                Location::new(scope.path(), self.id.span()),
            )),
        }
    }
}
