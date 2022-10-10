use std::rc::Rc;

use crate::ast;
use crate::dst;
use crate::dst::InferType;
use crate::scope::Scope;

pub trait Resolve<T> {
    fn resolve(&self, scope: &dyn Scope) -> Result<T, String>;
}

impl Resolve<dst::Module> for ast::Module {
    // TODO: `scope` is the program.
    fn resolve(&self, _scope: &dyn Scope) -> Result<dst::Module, String> {
        let mut dst_module = dst::Module::default();

        for body in &self.body {
            match body {
                ast::BlockBody::Statement(stmt) => match stmt {
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
                        return Err("Unused expression result".to_string());
                    }

                    dst_module.main.push(dst::Statement::TerminatedExpr(expr));
                }
            }
        }

        Ok(dst_module)
    }
}

impl Resolve<Rc<dst::Expr>> for ast::Expr {
    fn resolve(&self, scope: &dyn Scope) -> Result<Rc<dst::Expr>, String> {
        match self {
            ast::Expr::BoolLiteral(b) => Ok(Rc::new(dst::Expr::BoolLiteral(*b))),
            ast::Expr::IdRef(id) => {
                if let Some(var) = scope.find(id) {
                    Ok(Rc::new(dst::Expr::VarRef(Rc::clone(&var))))
                } else {
                    Err(format!("Unknown variable: {}", id))
                }
            }
            ast::Expr::MacroCall(m) => Ok(Rc::new(dst::Expr::MacroCall(m.resolve(scope)?))),
        }
    }
}

impl Resolve<Rc<dst::VarDecl>> for ast::VarDecl {
    /// Pushes the resolved variable declaration to the scope.
    fn resolve(&self, scope: &dyn Scope) -> Result<Rc<dst::VarDecl>, String> {
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
    fn resolve(&self, scope: &dyn Scope) -> Result<dst::MacroCall, String> {
        match self.id.as_str() {
            "assert" => {
                assert_eq!(self.args.len(), 1);
                let arg = &self.args[0].resolve(scope)?;
                Ok(dst::MacroCall::Assert(Rc::clone(arg)))
            }
            _ => panic!("Unknown macro @{}", self.id),
        }
    }
}
