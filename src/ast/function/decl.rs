use std::fmt::Display;

use crate::{
    ast::Qualifier,
    location::{HasSpan, Span},
};

use super::Param;

#[derive(Debug, PartialEq, Clone)]
pub struct Decl {
    span: Span,
    pub r#pub: bool,
    pub id: Qualifier,
    pub params: Vec<Param>,
    pub return_type: Qualifier,
}

impl Decl {
    pub fn new(
        span: Span,
        r#pub: bool,
        id: Qualifier,
        params: Vec<Param>,
        return_type: Qualifier,
    ) -> Self {
        Self {
            span,
            r#pub,
            id,
            params,
            return_type,
        }
    }
}

impl HasSpan for Decl {
    fn span(&self) -> Span {
        self.span
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.r#pub {
            write!(f, "pub ")?;
        }

        write!(f, "fn {}(", self.id)?;

        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", param)?;
        }

        write!(f, ") -> {}", self.return_type)
    }
}
