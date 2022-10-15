use std::fmt::{Debug, Display, Formatter};

use crate::location::Location;

pub struct Note {
    pub message: String,
    pub location: Option<Location>,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = &self.location {
            write!(f, "{} at {}", self.message, location,)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

/// An Onyx compiler panic.
pub struct Panic {
    pub message: String,
    pub location: Option<Location>,
    pub notes: Vec<Note>,
}

impl Panic {
    // TODO: Pass notes here, still allow to add later.
    pub fn new(message: String, location: Option<Location>) -> Self {
        Self {
            message,
            location,
            notes: Vec::new(),
        }
    }

    pub fn add_note(&mut self, message: String, location: Option<Location>) {
        self.notes.push(Note { message, location });
    }
}

impl Display for Panic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[41m \x1b[1m  PANIC  \x1b[0m {}", self.message)?;

        if let Some(location) = &self.location {
            write!(f, " at {}", location)?;
        }

        for note in &self.notes {
            writeln!(f, "\x1b[45m \x1b[1m  NOTE  \x1b[0m {}", note)?;
        }

        Ok(())
    }
}

impl Debug for Panic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Panic! {}", self.message)?;

        if let Some(location) = &self.location {
            write!(f, " at {}", location)?;
        }

        for note in &self.notes {
            write!(f, ". Note: {}", note)?;
        }

        Ok(())
    }
}
