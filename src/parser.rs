use crate::{ast, location, Location, Panic};

struct Node<T> {
    span: location::Span,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T, offset_start: usize, offset_end: usize) -> Self {
        Self {
            value,
            span: location::Span {
                start: location::Cursor::incomplete(offset_start),
                end: location::Cursor::incomplete(offset_end),
            },
        }
    }
}

peg::parser! {
  grammar onyx_parser() for str {
    pub rule start() -> ast::Mod
        = ___? body:block_body() ___? { ast::Mod::new(body) }

    // Utils ==================================================================
    //

    /// A wrapper yielding `Node` with a span.
    rule node<T>(r: rule<T>) -> Node<T>
        = start:position!()
          it:r()
          end:position!()
        { Node::new(it, start, end) }

    // Punctuation ============================================================
    //

    /// Horizontal space.
    rule sp() = [' ' | '\t']+
    rule _ = sp()

    /// A single newline.
    rule nl() = _? ("\r\n" / "\n" / "\r") _?

    /// "Non-adjacent", i.e. horizontal space or a single newline.
    rule nadj() = nl() / _
    rule __ = nadj()

    /// "Wide-space", i.e. any amount of space (including multiple lines).
    rule wsp() = nl()+ / _
    rule ___ = wsp()

    rule eof() = ![_]
    rule term() = nl() / (_? (";" / &eof() / &"}" / &"]" / &")"))

    // Atoms ==================================================================
    //

    /// An Onyx idetifier.
    rule id() -> &'input str
        = $(
            ("_" / ['a'..='z' | 'A'..='Z'])
            ("_" / ['a'..='z' | 'A'..='Z' | '0'..='9'])*
        )

    /// A boolean literal.
    rule bool() -> bool = "true"  { true } / "false" { false }

    // Expressions ============================================================
    //

    /// An expression.
    rule expr() -> ast::Expr = precedence! {
        l:@ _? op:"=" _? r:(@) {
            ast::Expr::Binop(ast::Binop::new(l, "=".to_string(), r))
        }
        --
        it:macro_call() { ast::Expr::MacroCall(it) }
        n:node(<bool()>) {
            ast::Expr::BoolLiteral(ast::Bool::new(n.span, n.value))
        }
        n:node(<id()>)   {
            ast::Expr::IdRef(ast::Id::new(n.span, n.value.to_string()))
        }
    }

    /// A macro call.
    rule macro_call() -> ast::MacroCall
        =
            start:position!()
            "@" id:node(<id()>) "(" args:(expr() ** ",") ")"
            end:position!()
        {
            ast::MacroCall::new(
                location::Span::incomplete(start, end),
                ast::Node::new(id.span, id.value.to_string()),
                args
            )
        }

    // Statements =============================================================
    //

    /// A comment spans until the end of the line.
    rule comment() -> ast::Comment
        =
            start:position!() "#"
            text:$([^ '\n' | '\r']*) (&nl() / &eof())
            end:position!()
        {
            ast::Comment::new(
                location::Span::incomplete(start, end),
                text.to_string()
            )
        }

    /// An expression explicitly terminated w/ `;`.
    rule terminated_expr() -> ast::Statement
        = it:expr() _? ";"
        { ast::Statement::TerminatedExpr(it) }

    rule var_decl_value() -> ast::Expr
        = _? "=" __? expr:expr() { expr }

    /// A variable declaration.
    rule var_decl() -> ast::VarDecl
        =
            start:position!()
            "let" _ id:node(<id()>) expr:var_decl_value() term()
            end:position!()
        {
            ast::VarDecl::new(
                location::Span::incomplete(start, end),
                ast::Id::new(id.span, id.value.to_string()),
                expr
            )
        }

    /// A statement.
    rule statement() -> ast::Statement
        = it:var_decl() { ast::Statement::VarDecl(it) }
        / terminated_expr()

    rule block_body_el() -> ast::BlockBody
        = it:comment()  { ast::BlockBody::Comment(it) }
        / it:statement() { ast::BlockBody::Stmt(it) }
        / it:expr()      { ast::BlockBody::Expr(it) }

    /// A block body, i.e. a sequence of statements and expressions.
    rule block_body() -> Vec<ast::BlockBody>
        = ___? body:block_body_el() ** (___?)
        { body }
  }
}

impl From<peg::str::LineCol> for location::Span {
    fn from(lc: peg::str::LineCol) -> Self {
        Self::thin(location::Cursor::new(lc.offset, lc.line - 1, lc.column - 1))
    }
}

pub fn parse(path: &str, source: &str) -> Result<ast::Mod, Panic> {
    match onyx_parser::start(source) {
        Ok(result) => Ok(result),
        Err(err) => Err(Panic::new(
            format!("Expected {}", err.expected),
            Location::new(path.to_string(), err.location.into()),
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast;
    use crate::location::Span;

    #[test]
    pub fn test_basic() {
        let input1 = r#"let x = true; @assert(x)"#;

        let ast = ast::Mod {
            body: vec![
                ast::BlockBody::Stmt(ast::Statement::VarDecl(ast::VarDecl::new(
                    Span::incomplete(0, 13),
                    ast::Id::new(Span::incomplete(4, 5), "x".to_string()),
                    ast::Expr::BoolLiteral(ast::Bool::new(Span::incomplete(8, 12), true)),
                ))),
                ast::BlockBody::Expr(ast::Expr::MacroCall(ast::MacroCall::new(
                    Span::incomplete(14, 25),
                    ast::Node::new(Span::incomplete(15, 21), "assert".to_string()),
                    vec![ast::Expr::IdRef(ast::Id::new(
                        Span::incomplete(22, 23),
                        "x".to_string(),
                    ))],
                ))),
            ],
        };

        assert_eq!(parse("", input1).as_ref().unwrap(), &ast);
        // assert_eq!(parse("", input2).as_ref().unwrap(), &ast);
        // assert_eq!(parse("", input3).as_ref().unwrap(), &ast);
    }

    #[test]
    pub fn text_comment() {
        let input = r#"# this is a comment"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Comment(ast::Comment::new(
                Span::incomplete(0, 19),
                " this is a comment".to_string(),
            ))],
        };

        assert_eq!(parse("", input).as_ref().unwrap(), &ast);
    }

    #[test]
    pub fn test_binop() {
        let input1 = r#"a = b"#;

        let ast = ast::Mod {
            body: vec![ast::BlockBody::Expr(ast::Expr::Binop(ast::Binop::new(
                ast::Expr::IdRef(ast::Id::new(Span::incomplete(0, 1), "a".to_string())),
                "=".to_string(),
                ast::Expr::IdRef(ast::Id::new(Span::incomplete(4, 5), "b".to_string())),
            )))],
        };

        assert_eq!(parse("", input1).as_ref().unwrap(), &ast);
    }
}
