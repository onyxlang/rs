use crate::ast;

peg::parser! {
  pub grammar onyx_parser() for str {
    /// Horizontal space.
    rule sp() = [' ' | '\t']+
    rule _ = sp()

    /// A single newline.
    rule nl() = _? ("\r\n" / "\n" / "\r") _?

    /// "Non-adjacent", i.e. space or a single newline.
    rule nadj() = nl() / _
    rule __ = nadj()

    /// "Wide-space", i.e. any amount of space.
    rule wsp() = nl()+ / _
    rule ___ = wsp()

    rule eof() = ![_]

    rule term() = nl() / (_? (";" / &eof() / &"}" / &"]" / &")"))

    rule id() -> &'input str
        = $(
            ("_" / ['a'..='z' | 'A'..='Z'])
            ("_" / ['a'..='z' | 'A'..='Z' | '0'..='9'])*
        )

    rule bool() -> bool
        = "true"  { true }
        / "false" { false }

    rule terminated_expr() -> ast::Statement
        = expr:expr() _? ";"
        { ast::Statement::TerminatedExpr(expr) }

    rule expr() -> ast::Expr
        = a:bool()       { ast::Expr::BoolLiteral(a) }
        / a:id()         { ast::Expr::IdRef(a.to_string()) }
        / a:macro_call() { ast::Expr::MacroCall(a) }

    rule var_decl_value() -> ast::Expr
        = _? "=" __? expr:expr() { expr }

    rule var_decl() -> ast::VarDecl
        = "let" _ id:id() expr:var_decl_value() term()
        { ast::VarDecl { id: id.to_string(), expr } }

    rule statement() -> ast::Statement
        = a:var_decl()      { ast::Statement::VarDecl(a) }
        / terminated_expr()

    rule block_body_el() -> ast::BlockBody
        = a:statement() { ast::BlockBody::Statement(a) }
        / a:expr()      { ast::BlockBody::Expr(a) }

    rule block_body() -> Vec<ast::BlockBody>
        = ___? body:block_body_el() ** (___?)
        { body }

    rule macro_call() -> ast::MacroCall
        = "@" id:id() "(" args:(expr() ** ",") ")"
        { ast::MacroCall { id: id.to_string(), args } }

    pub rule start() -> ast::Module
        = ___? body:block_body() ___? { ast::Module::new(body) }
  }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast;

    #[test]
    pub fn test_basic() {
        let input1 = r#"let x = true; @assert(x)"#;

        let input2 = r#"
          let x = true
          @assert(x)"#;

        let input3 = r#"
          let x = true; 
          @assert(x)"#;

        let ast = ast::Module {
            body: vec![
                ast::BlockBody::Statement(ast::Statement::VarDecl(ast::VarDecl {
                    id: "x".to_string(),
                    expr: ast::Expr::BoolLiteral(true),
                })),
                ast::BlockBody::Expr(ast::Expr::MacroCall(ast::MacroCall {
                    id: "assert".to_string(),
                    args: vec![ast::Expr::IdRef("x".to_string())],
                })),
            ],
        };

        assert_eq!(onyx_parser::start(input1).as_ref(), Ok(&ast));
        assert_eq!(onyx_parser::start(input2).as_ref(), Ok(&ast));
        assert_eq!(onyx_parser::start(input3).as_ref(), Ok(&ast));
    }
}
