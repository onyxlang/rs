use std::fmt::Display;

use crate::{
    ast::{Id, Qualifier},
    location::Span,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub span: Span,
    pub id: Id,
    pub r#type: Qualifier,
}

impl Param {
    pub fn new(span: Span, id: Id, r#type: Qualifier) -> Self {
        Self { span, id, r#type }
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.r#type)
    }
}
