use crate::{ast::Id, location::Span};

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub span: Span,
    pub id: Id,
    pub r#type: Id,
}

impl Param {
    pub fn new(span: Span, id: Id, r#type: Id) -> Self {
        Self { span, id, r#type }
    }
}
