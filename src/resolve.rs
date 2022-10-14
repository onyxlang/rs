use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use crate::ast;
use crate::dst;
use crate::dst::InferType;
use crate::dst::Scope;
use crate::location::HasSpan;
use crate::panic::Panic;
use crate::unit::Unit;
use crate::Location;

pub trait Resolve<T> {
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<T, Panic>;
}

impl ast::Mod {
    pub fn resolve(&self, unit: Weak<RefCell<Unit>>) -> Result<dst::Mod, Panic> {
        let mut dst_module = dst::Mod::new(unit);

        for body in &self.body {
            match body {
                ast::BlockBody::Stmt(stmt) => match stmt {
                    ast::Statement::VarDecl(var_decl) => {
                        let var = var_decl.resolve(&mut dst_module)?;
                        dst_module.store(dst::Exportable::VarDecl(Rc::clone(&var)));
                        dst_module.main.push(dst::Statement::VarDecl(var));
                    }
                    ast::Statement::TerminatedExpr(expr) => {
                        let expr = expr.resolve(&mut dst_module)?;
                        dst_module.main.push(dst::Statement::TerminatedExpr(expr));
                    }
                    ast::Statement::Import(i) => {
                        let import = i.resolve(&mut dst_module)?;
                        dst_module.add_import(&i.id.value, import);
                    }
                    ast::Statement::Decorator(d) => {
                        let decorator = d.resolve(&mut dst_module)?;
                        dst_module.push_decorator(decorator);
                    }
                    ast::Statement::StructDef(def) => {
                        let decl = def.resolve(&mut dst_module)?;
                        dst_module.store(dst::Exportable::StructDecl(Rc::clone(&decl)));

                        if def.export {
                            if def.default {
                                dst_module.default =
                                    Some(dst::Exportable::StructDecl(Rc::clone(&decl)));
                            } else {
                                dst_module
                                    .add_export(&def.id.value, dst::Exportable::StructDecl(decl));
                            }
                        }
                    }
                },
                ast::BlockBody::Expr(expr) => {
                    let expr = expr.resolve(&mut dst_module)?;

                    if expr.infer_type() != dst::BuiltinType::Void {
                        return Err(Panic::new(
                            "Unused expression result".to_string(),
                            Some(Location::new(dst_module.path(), expr.span())),
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

impl Resolve<dst::Import> for ast::Import {
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<dst::Import, Panic> {
        if scope.search(&self.id.value).is_some() {
            let panic = Panic::new(
                format!("Already declared id `{}`", &self.id.value),
                Some(Location::new(scope.path(), self.id.span())),
            );

            // TODO: Point to the previous declaration, which may be an import.
            return Err(panic);
        }

        scope.resolve_import(self.clone())
    }
}

impl Resolve<dst::decorator::Application> for ast::Decorator {
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<dst::decorator::Application, Panic> {
        match self.id.value.as_str() {
            "Builtin" => Ok(dst::decorator::Application::new(
                self.clone(),
                dst::decorator::Builtin::Builtin,
            )),

            // TODO: Lookup for the decorator definition in the scope.
            _ => Err(Panic::new(
                format!("Unknown decorator `{}`", &self.id.value),
                Some(Location::new(scope.path(), self.id.span())),
            )),
        }
    }
}

impl Resolve<Rc<RefCell<dst::r#struct::Decl>>> for ast::r#struct::Def {
    fn resolve(
        &self,
        scope: &mut dyn dst::Scope,
    ) -> Result<Rc<RefCell<dst::r#struct::Decl>>, Panic> {
        let mut builtin: Option<dst::r#struct::Builtin> = None;

        for decorator in scope.pop_decorators() {
            match decorator {
                dst::decorator::Application {
                    decorator: dst::decorator::Builtin::Builtin,
                    ..
                } => {
                    if builtin.is_some() {
                        return Err(Panic::new(
                            "Duplicate decorator `Builtin`".to_string(),
                            Some(Location::new(scope.path(), self.id.span())),
                        ));
                    }

                    match self.id.value.as_str() {
                        "Bool" => {
                            builtin = Some(dst::r#struct::Builtin::Bool);
                        }
                        &_ => {
                            return Err(Panic::new(
                                format!("Unknown builtin struct `{}`", &self.id.value),
                                Some(Location::new(scope.path(), self.id.span())),
                            ))
                        }
                    }
                }
            }
        }

        let decl = Rc::new(RefCell::new(dst::r#struct::Decl::new(
            self.clone(),
            builtin,
        )));

        let r#impl = Rc::new(dst::r#struct::Impl::new(Rc::downgrade(&decl)));
        decl.borrow_mut().add_impl(r#impl);

        Ok(decl)
    }
}

impl Resolve<Rc<dst::Expr>> for ast::Expr {
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<Rc<dst::Expr>, Panic> {
        match self {
            ast::Expr::BoolLiteral(b) => Ok(Rc::new(dst::Expr::BoolLiteral(b.clone()))),
            ast::Expr::IdRef(id) => {
                if let Some(ent) = scope.search(id.value.as_str()) {
                    match ent {
                        dst::Exportable::VarDecl(var) => Ok(Rc::new(dst::Expr::VarRef(
                            dst::VarRef::new(id.clone(), Rc::clone(&var)),
                        ))),
                        dst::Exportable::StructDecl(_) => todo!(),
                    }
                } else {
                    Err(Panic::new(
                        format!("Unknown identifier: {}", id.value),
                        Some(Location::new(scope.path(), id.span())),
                    ))
                }
            }
            ast::Expr::MacroCall(m) => Ok(Rc::new(dst::Expr::MacroCall(m.resolve(scope)?))),
            ast::Expr::Binop(b) => match b.op.as_str() {
                "=" => {
                    let lhs = b.lhs.resolve(scope)?;
                    let rhs = b.rhs.resolve(scope)?;

                    if let dst::Expr::VarRef(r#ref) = &*lhs {
                        if r#ref.decl.r#type != rhs.infer_type() {
                            return Err(Panic::new(
                                format!(
                                    "Type mismatch: expected {}, got {}",
                                    r#ref.decl.r#type,
                                    rhs.infer_type()
                                ),
                                Some(Location::new(scope.path(), rhs.span())),
                            ));
                        }

                        Ok(Rc::new(dst::Expr::Assignment(dst::Assignment {
                            lhs: r#ref.clone(),
                            rhs,
                        })))
                    } else {
                        Err(Panic::new(
                            "Left-hand side of assignment must be a variable".to_string(),
                            Some(Location::new(scope.path(), lhs.span())),
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
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<Rc<dst::VarDecl>, Panic> {
        // TODO: Apply decorators.
        let expr = self.expr.resolve(scope)?;
        let r#type = expr.infer_type();
        let var = Rc::new(dst::VarDecl::new(self.clone(), r#type, expr));
        Ok(var)
    }
}

impl Resolve<dst::MacroCall> for ast::MacroCall {
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<dst::MacroCall, Panic> {
        match self.id.value.as_str() {
            "assert" => {
                assert_eq!(self.args.len(), 1);
                let arg = &self.args[0].resolve(scope)?;
                Ok(dst::MacroCall::Assert(self.clone(), Rc::clone(arg)))
            }
            _ => Err(Panic::new(
                format!("Unknown macro: {}", self.id.value),
                Some(Location::new(scope.path(), self.id.span())),
            )),
        }
    }
}
