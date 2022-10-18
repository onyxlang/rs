use std::{cell::RefCell, rc::Rc};

use crate::{ast, location::Cursor, location::Span, unit::Unit, Location, Panic};

macro_rules! span {
    ($begin: expr, $end: expr) => {
        Span::new(Cursor::new($begin), Cursor::new($end))
    };
}

peg::parser! {
  grammar onyx_parser() for str {
    pub rule start() -> ast::Mod
        = ___? body:block_body() ___? { ast::Mod::new(body) }

    // Punctuation ============================================================
    //

    /// Horizontal space.
    rule sp() = quiet!{ [' ' | '\t']+ }
    rule _ = sp()

    /// A single newline.
    rule nl() = quiet!{ _? ("\r\n" / "\n" / "\r") _? }

    /// "Non-adjacent", i.e. horizontal space or a single newline.
    rule nadj() = nl() / _
    rule __ = nadj()

    /// "Wide-space", i.e. any amount of space (including multiple lines).
    rule wsp() = quiet!{ nl()+ / _ }
    rule ___ = wsp()

    rule eof() = ![_]
    rule term() = quiet!{ nl() / (_? (";" / &eof() / &"}" / &"]" / &")")) }
        / expected!("terminator")

    // Atoms ==================================================================
    //

    /// An Onyx idetifier.
    rule id() -> ast::Id
        =
            begin:position!()
            value:$(quiet!{
                ("_" / ['a'..='z' | 'A'..='Z'])
                ("_" / ['a'..='z' | 'A'..='Z' | '0'..='9'])*
                "?"?
            } / expected!("identifier"))
            end:position!()
        { ast::Id::new(span!(begin, end), value.to_string()) }

    /// A boolean literal.
    rule bool() -> ast::literal::Bool
        = begin:position!() value:$("true" / "false") end:position!()
        { ast::literal::Bool::new(span!(begin, end), value == "true") }

    /// A string literal.
    rule string() -> ast::literal::String
        = begin:position!() "\"" string:$((!"\"" [_])*) "\"" end:position!()
        { ast::literal::String::new(span!(begin, end), string.to_string()) }

    // Expressions ============================================================
    //

    rule qualifier() -> ast::Qualifier
        =
            begin:position!()
            id:id()
            end:position!()
        { ast::Qualifier::new(span!(begin, end), id) }

    /// An expression.
    rule expr() -> ast::Expr = precedence! {
        it:call() { ast::Expr::FunctionCall(it) }
        --
        l:@ _? op:"=" _? r:(@) {
            ast::Expr::Binop(ast::Binop::new(l, "=".to_string(), r))
        }
        --
        it:macro_call() { ast::Expr::MacroCall(it) }
        it:bool()       { ast::Expr::BoolLiteral(it) }
        it:qualifier()  { ast::Expr::Ref(it) }
    }

    rule args() -> Vec<ast::Expr>
        = "(" ___? args:(expr() ** ("," ___?)) ___? ")" { args }

    rule call() -> ast::Call
        = begin:position!() callee:qualifier() args:args() end:position!()
        { ast::Call::new(span!(begin, end), callee, args) }

    /// A macro call.
    rule macro_call() -> ast::MacroCall
        =
            begin:position!()
            "@" id:id() "(" args:(expr() ** ",") ")"
            end:position!()
        { ast::MacroCall::new(span!(begin, end), id, args) }

    // Statements =============================================================
    //

    /// A comment spans until the end of the line.
    rule comment() -> ast::Comment
        =
            begin:position!() "#"
            text:$([^ '\n' | '\r']*) (&nl() / &eof())
            end:position!()
        { ast::Comment::new(span!(begin, end), text.to_string()) }

    /// An `import` statement.
    rule import() -> ast::Import
        =
            begin:position!()
            r#pub:("pub" _)?
            "import" _? "{" ___? ids:(id() ** ("," ___?)) ___? "}" _?
            "from" _ from:string()
            end:position!()
        { ast::Import::new(span!(begin, end), r#pub.is_some(), ids, from) }

    /// A decorator, e.g. `@[Foo]`.
    rule decorator() -> ast::Decorator
        = begin:position!() "@[" id:id() "]" end:position!()
        { ast::Decorator::new(span!(begin, end), id) }

    /// An expression explicitly terminated w/ `;`.
    rule terminated_expr() -> ast::Statement
        = it:expr() _? ";"
        { ast::Statement::TerminatedExpr(it) }

    rule var_decl_value() -> ast::Expr
        = _? "=" __? expr:expr() { expr }

    /// A variable declaration.
    rule var_decl() -> ast::VarDecl
        =
            begin:position!()
            "let" _ id:id() expr:var_decl_value() term()
            end:position!()
        { ast::VarDecl::new(span!(begin, end), id, expr) }

    /// A struct definition.
    rule struct_def() -> ast::r#struct::Def
        =
            begin:position!()
            r#pub:("pub" _)?
            "struct" _ id:id() _? "{" ___? "}"
            end:position!()
        {
            ast::r#struct::Def::new(
                span!(begin, end),
                r#pub.is_some(),
                id,
            )
        }

    rule function_param() -> ast::function::Param
        =
            begin:position!()
            id:id() _?
            ":" __?
            r#type:qualifier()
            end:position!()
        { ast::function::Param::new(span!(begin, end), id, r#type) }

    /// A function declaration.
    rule function_decl() -> ast::function::Decl
        =
            begin:position!()
            r#pub:("pub" _)?
            "fn" _
            id:qualifier() _?
            "(" ___? params:function_param() ** ("," ___?) ")" _?
            "->" __? return_type:qualifier()
            term()
            end:position!()
        {
            ast::function::Decl::new(
                span!(begin, end),
                r#pub.is_some(),
                id,
                params,
                return_type
            )
        }

    /// A statement.
    rule statement() -> ast::Statement
        = it:var_decl()      { ast::Statement::VarDecl(it) }
        / it:import()        { ast::Statement::Import(it) }
        / it:decorator()     { ast::Statement::Decorator(it) }
        / it:struct_def()    { ast::Statement::StructDef(it) }
        / it:function_decl() { ast::Statement::FunctionDecl(it) }
        / terminated_expr()

    rule block_body_el() -> ast::BlockBody
        = it:comment()   { ast::BlockBody::Comment(it) }
        / it:statement() { ast::BlockBody::Stmt(it) }
        / it:expr()      { ast::BlockBody::Expr(it) }

    /// A block body, i.e. a sequence of statements and expressions.
    rule block_body() -> Vec<ast::BlockBody>
        = ___? body:block_body_el() ** (___?)
        { body }
  }
}

impl From<peg::str::LineCol> for Span {
    fn from(lc: peg::str::LineCol) -> Self {
        Self::thin(Cursor::new(lc.offset))
    }
}

pub fn parse(unit: Rc<RefCell<Unit>>) -> Result<ast::Mod, Panic> {
    match onyx_parser::start(&unit.as_ref().borrow_mut().try_source()?) {
        Ok(result) => Ok(result),
        Err(err) => Err(Panic::new(
            format!("Expected {}", err.expected),
            Some(Location::new(unit.clone(), err.location.into())),
        )),
    }
}

/// Rust-panics upon failure.
pub fn parse_simple(source: &str) -> ast::Mod {
    onyx_parser::start(source).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast;
    use crate::location::Span;

    #[test]
    pub fn test_basic() {
        let input = r#"let x = true; @assert(x)"#;

        let ast = ast::Mod {
            body: vec![
                ast::BlockBody::Stmt(ast::Statement::VarDecl(ast::VarDecl::new(
                    span!(0, 13),
                    ast::Id::new(span!(4, 5), "x".to_string()),
                    ast::Expr::BoolLiteral(ast::literal::Bool::new(span!(8, 12), true)),
                ))),
                ast::BlockBody::Expr(ast::Expr::MacroCall(ast::MacroCall::new(
                    span!(14, 25),
                    ast::Id::new(span!(15, 21), "assert".to_string()),
                    vec![ast::Expr::Ref(ast::Qualifier::from_string(
                        span!(22, 23),
                        "x".to_string(),
                    ))],
                ))),
            ],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn text_comment() {
        let input = r#"# this is a comment"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Comment(ast::Comment::new(
                span!(0, 19),
                " this is a comment".to_string(),
            ))],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn test_binop() {
        let input = r#"a = b"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Expr(ast::Expr::Binop(ast::Binop::new(
                ast::Expr::Ref(ast::Qualifier::from_string(span!(0, 1), "a".to_string())),
                "=".to_string(),
                ast::Expr::Ref(ast::Qualifier::from_string(span!(4, 5), "b".to_string())),
            )))],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn test_import() {
        let input = r#"pub import { Foo } from "bar""#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Stmt(ast::Statement::Import(
                ast::Import::new(
                    span!(0, 29),
                    true,
                    vec![ast::Id::new(span!(13, 16), "Foo".to_string())],
                    ast::literal::String::new(span!(24, 29), "bar".to_string()),
                ),
            ))],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn test_decorator() {
        let input = r#"@[Foo]"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Stmt(ast::Statement::Decorator(
                ast::Decorator::new(span!(0, 6), ast::Id::new(span!(2, 5), "Foo".to_string())),
            ))],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn test_struct_def() {
        let input = r#"pub struct Foo { }"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Stmt(ast::Statement::StructDef(
                ast::r#struct::Def::new(
                    span!(0, 19),
                    true,
                    ast::Id::new(span!(12, 15), "Foo".to_string()),
                ),
            ))],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn test_function_decl1() {
        let input = r#"pub fn eq?(a: Bool, b: Bool) -> Bool"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Stmt(ast::Statement::FunctionDecl(
                ast::function::Decl::new(
                    span!(0, 36),
                    true,
                    ast::Qualifier::from_string(span!(7, 10), "eq?".to_string()),
                    vec![
                        ast::function::Param::new(
                            span!(11, 18),
                            ast::Id::new(span!(11, 12), "a".to_string()),
                            ast::Qualifier::from_string(span!(14, 18), "Bool".to_string()),
                        ),
                        ast::function::Param::new(
                            span!(20, 27),
                            ast::Id::new(span!(20, 21), "b".to_string()),
                            ast::Qualifier::from_string(span!(23, 27), "Bool".to_string()),
                        ),
                    ],
                    ast::Qualifier::from_string(span!(32, 36), "Bool".to_string()),
                ),
            ))],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn test_function_decl2() {
        let input = r#"fn foo ()-> Bar"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Stmt(ast::Statement::FunctionDecl(
                ast::function::Decl::new(
                    span!(0, 15),
                    false,
                    ast::Qualifier::from_string(span!(3, 6), "foo".to_string()),
                    vec![],
                    ast::Qualifier::from_string(span!(12, 15), "Bar".to_string()),
                ),
            ))],
        };

        assert_eq!(parse_simple(input), ast);
    }

    #[test]
    pub fn test_call() {
        let input = r#"x(y, true)"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Expr(ast::Expr::FunctionCall(
                ast::Call::new(
                    span!(0, 10),
                    ast::Qualifier::from_string(span!(0, 1), "x".to_string()),
                    vec![
                        ast::Expr::Ref(ast::Qualifier::from_string(span!(2, 3), "y".to_string())),
                        ast::Expr::BoolLiteral(ast::literal::Bool::new(span!(5, 9), true)),
                    ],
                ),
            ))],
        };

        assert_eq!(parse_simple(input), ast);
    }
}
