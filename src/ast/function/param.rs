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
