use crate::location::{HasSpan, Span};
use std::fmt::{Debug, Formatter};

pub struct Comment {
    span: Span,
    pub text: String,
}

impl Comment {
    pub fn new(span: Span, text: String) -> Self {
        Self { span, text }
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Debug for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "##{}", self.text)
    }
}

impl HasSpan for Comment {
    fn span(&self) -> Span {
        self.span
    }
}
