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

mod qualifier;

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
                        dst_module.store(dst::Exportable::VarDecl(Rc::clone(&var)))?;
                        dst_module.main.push(dst::Statement::VarDecl(var));
                    }
                    ast::Statement::TerminatedExpr(expr) => {
                        let expr = expr.resolve(&mut dst_module)?;
                        dst_module.main.push(dst::Statement::TerminatedExpr(expr));
                    }
                    ast::Statement::Import(i) => {
                        let dep = dst_module.resolve_dependency(i.from.clone())?;

                        for id in &i.ids {
                            let export = dep
                                .as_ref()
                                .borrow()
                                .dst
                                .as_ref()
                                .unwrap()
                                .search(id)
                                .ok_or_else(|| {
                                    Panic::new(
                                        format!("{} not found in {}", id, i.from),
                                        Some(Location::new(dst_module.unit(), id.span())),
                                    )
                                })?;

                            if i.r#pub {
                                dst_module.exports.insert(id.value.clone(), export.clone());
                            }

                            dst_module.imports.insert(id.value.clone(), export);
                        }
                    }
                    ast::Statement::Decorator(d) => {
                        let decorator = d.resolve(&mut dst_module)?;
                        dst_module.push_decorator(decorator);
                    }
                    ast::Statement::StructDef(def) => {
                        let decl = def.resolve(&mut dst_module)?;
                        dst_module.store(dst::Exportable::StructDecl(Rc::clone(&decl)))?;

                        if def.r#pub {
                            dst_module
                                .exports
                                .insert(def.id.value.clone(), dst::Exportable::StructDecl(decl));
                        }
                    }
                    ast::Statement::FunctionDecl(decl) => {
                        let dst = decl.resolve(&mut dst_module)?;
                        dst_module.store(dst::Exportable::FunctionDecl(Rc::clone(&dst)))?;

                        if decl.r#pub {
                            dst_module.exports.insert(
                                decl.id.id.value.clone(),
                                dst::Exportable::FunctionDecl(dst),
                            );
                        }
                    }
                },
                ast::BlockBody::Expr(expr) => {
                    let expr = expr.resolve(&mut dst_module)?;

                    if expr.infer_type(&dst_module).is_some() {
                        return Err(Panic::new(
                            "Unused expression result".to_string(),
                            Some(Location::new(dst_module.unit(), expr.span())),
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

impl Resolve<dst::decorator::Application> for ast::Decorator {
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<dst::decorator::Application, Panic> {
        match self.id.value.as_str() {
            "Builtin" => Ok(dst::decorator::Application::new(
                self.clone(),
                dst::decorator::Builtin::Builtin,
            )),

            // TODO: Lookup for the decorator definition in the scope.
            _ => Err(Panic::new(
                format!("Unknown decorator {}", &self.id),
                Some(Location::new(scope.unit(), self.id.span())),
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
                            Some(Location::new(scope.unit(), self.id.span())),
                        ));
                    }

                    match self.id.value.as_str() {
                        "Bool" => {
                            builtin = Some(dst::r#struct::Builtin::Bool);
                        }
                        &_ => {
                            return Err(Panic::new(
                                format!("Unknown builtin struct {}", &self.id),
                                Some(Location::new(scope.unit(), self.id.span())),
                            ))
                        }
                    }
                }
            }
        }

        if builtin.is_none() {
            unimplemented!("Non-builtin struct");
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

impl Resolve<Rc<RefCell<dst::function::Decl>>> for ast::function::Decl {
    fn resolve(
        &self,
        scope: &mut dyn dst::Scope,
    ) -> Result<Rc<RefCell<dst::function::Decl>>, Panic> {
        let mut builtin: Option<dst::function::Builtin> = None;

        for decorator in scope.pop_decorators() {
            match decorator {
                dst::decorator::Application {
                    decorator: dst::decorator::Builtin::Builtin,
                    ..
                } => {
                    if builtin.is_some() {
                        return Err(Panic::new(
                            "Duplicate decorator `Builtin`".to_string(),
                            Some(Location::new(scope.unit(), self.id.span())),
                        ));
                    }

                    match self.id.id.value.as_str() {
                        "eq?" => {
                            builtin = Some(dst::function::Builtin::BoolEq);
                        }
                        &_ => {
                            return Err(Panic::new(
                                format!("Unknown builtin function {}", &self.id),
                                Some(Location::new(scope.unit(), self.id.span())),
                            ))
                        }
                    }
                }
            }
        }

        if builtin.is_none() {
            unimplemented!("Non-builtin function");
        }

        let mut params: Vec<dst::function::decl::Param> = vec![];

        for param in &self.params {
            let param =
                dst::function::decl::Param::new(param.id.clone(), param.r#type.resolve(scope)?);
            params.push(param);
        }

        let return_type = self.return_type.resolve(scope)?;

        let decl = Rc::new(RefCell::new(dst::function::Decl::new(
            self.clone(),
            builtin,
            params,
            Some(return_type),
        )));

        Ok(decl)
    }
}

impl Resolve<Rc<dst::Expr>> for ast::Expr {
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<Rc<dst::Expr>, Panic> {
        match self {
            ast::Expr::BoolLiteral(b) => Ok(Rc::new(dst::Expr::BoolLiteral(b.clone()))),
            ast::Expr::Ref(id) => {
                if let Some(ent) = scope.search(&id.id) {
                    match ent {
                        dst::Exportable::VarDecl(var) => Ok(Rc::new(dst::Expr::VarRef(
                            dst::VarRef::new(id.id.clone(), Rc::clone(&var)),
                        ))),
                        dst::Exportable::StructDecl(_) => Err(Panic::new(
                            format!("Cannot use struct {} as a value", id),
                            Some(Location::new(scope.unit(), id.span())),
                        )),
                        dst::Exportable::FunctionDecl(_) => Err(Panic::new(
                            format!("Cannot use function {} as a value", id),
                            Some(Location::new(scope.unit(), id.span())),
                        )),
                    }
                } else {
                    Err(Panic::new(
                        format!("Invalid reference {}", id),
                        Some(Location::new(scope.unit(), id.span())),
                    ))
                }
            }
            ast::Expr::MacroCall(m) => Ok(Rc::new(dst::Expr::MacroCall(m.resolve(scope)?))),
            ast::Expr::Binop(b) => match b.op.as_str() {
                "=" => {
                    let lhs = b.lhs.resolve(scope)?;
                    let rhs = b.rhs.resolve(scope)?;

                    if let dst::Expr::VarRef(r#ref) = &*lhs {
                        let rhs_type = rhs.infer_type(scope);

                        if rhs_type.is_none() {
                            return Err(Panic::new(
                                "Expression result must not be void".to_string(),
                                Some(Location::new(scope.unit(), b.span())),
                            ));
                        }

                        if r#ref.decl.r#type != *rhs_type.as_ref().unwrap() {
                            return Err(Panic::new(
                                format!(
                                    "Type mismatch: left is {}, right is {}",
                                    r#ref.decl.r#type.as_ref().borrow(),
                                    rhs_type.unwrap().as_ref().borrow()
                                ),
                                Some(Location::new(scope.unit(), rhs.span())),
                            ));
                        }

                        Ok(Rc::new(dst::Expr::Assignment(dst::Assignment {
                            lhs: r#ref.clone(),
                            rhs,
                        })))
                    } else {
                        Err(Panic::new(
                            "Left-hand side of assignment must be a variable".to_string(),
                            Some(Location::new(scope.unit(), lhs.span())),
                        ))
                    }
                }
                &_ => todo!(),
            },
            ast::Expr::FunctionCall(call) => {
                let callee: Rc<RefCell<dst::function::Decl>> = call.callee.resolve(scope)?;
                let mut args: Vec<Rc<dst::Expr>> = vec![];

                for arg in &call.args {
                    args.push(arg.resolve(scope)?);
                }

                let dstn = dst::Call::new(call.clone(), callee, args);
                Ok(Rc::new(dst::Expr::FunctionCall(dstn)))
            }
        }
    }
}

impl Resolve<Rc<dst::VarDecl>> for ast::VarDecl {
    /// Pushes the resolved variable declaration to the scope.
    fn resolve(&self, scope: &mut dyn dst::Scope) -> Result<Rc<dst::VarDecl>, Panic> {
        // TODO: Apply decorators.
        let expr = self.expr.resolve(scope)?;
        let r#type = expr.infer_type(scope);

        if r#type.is_none() {
            return Err(Panic::new(
                "Expression returns void".to_string(),
                Some(Location::new(scope.unit(), expr.span())),
            ));
        }

        let var = Rc::new(dst::VarDecl::new(self.clone(), r#type.unwrap(), expr));
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
                Some(Location::new(scope.unit(), self.id.span())),
            )),
        }
    }
}
