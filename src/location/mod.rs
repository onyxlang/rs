use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

use crate::unit::Unit;
pub use cursor::Cursor;
pub use span::HasSpan;
pub use span::Span;

mod cursor;
pub mod source;
mod span;

#[derive(Clone)]
pub struct Location {
    pub unit: Rc<RefCell<Unit>>,
    pub span: Span,
}

impl Location {
    pub fn new(unit: Rc<RefCell<Unit>>, span: Span) -> Self {
        Self { unit, span }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let span = source::Span::new(self.span, &self.unit.as_ref().borrow().source());

        // BUG: Breaks if path contains invalid Unicode.
        write!(f, "{}:{}", self.unit.as_ref().borrow().path.display(), span)
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
