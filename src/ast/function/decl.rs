use crate::{
    ast::Id,
    location::{HasSpan, Span},
};

use super::Param;

#[derive(Debug, PartialEq, Clone)]
pub struct Decl {
    span: Span,
    pub export: bool,
    pub default: bool,
    pub id: Id,
    pub params: Vec<Param>,
    pub return_type: Id,
}

impl Decl {
    pub fn new(
        span: Span,
        export: bool,
        default: bool,
        id: Id,
        params: Vec<Param>,
        return_type: Id,
    ) -> Self {
        Self {
            span,
            export,
            default,
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
