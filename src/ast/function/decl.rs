use crate::{
    ast::Qualifier,
    location::{HasSpan, Span},
};

use super::Param;

#[derive(Debug, PartialEq, Clone)]
pub struct Decl {
    span: Span,
    pub export: bool,
    pub default: bool,
    pub id: Qualifier,
    pub params: Vec<Param>,
    pub return_type: Qualifier,
}

impl Decl {
    pub fn new(
        span: Span,
        export: bool,
        default: bool,
        id: Qualifier,
        params: Vec<Param>,
        return_type: Qualifier,
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
