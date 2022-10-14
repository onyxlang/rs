use super::Decl;
use std::{cell::RefCell, rc::Weak};

/// A struct implementation node.
#[derive(Debug)]
pub struct Impl {
    pub decl: Weak<RefCell<Decl>>,
}

impl Impl {
    pub fn new(decl: Weak<RefCell<Decl>>) -> Self {
        Self { decl }
    }
}
