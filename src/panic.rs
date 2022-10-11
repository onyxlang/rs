use std::fmt::{Debug, Display, Formatter};

use crate::location::Location;

pub struct Note {
    pub message: String,
    pub location: Location,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.message, self.location)
    }
}

/// An Onyx compiler panic.
pub struct Panic {
    pub message: String,
    pub location: Location,
    pub notes: Vec<Note>,
}

impl Panic {
    pub fn new(message: String, location: Location) -> Self {
        Self {
            message,
            location,
            notes: Vec::new(),
        }
    }

    pub fn add_note(&mut self, message: String, location: Location) {
        self.notes.push(Note { message, location });
    }
}

impl Display for Panic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\x1b[41m \x1b[1m  PANIC  \x1b[0m {} at {}",
            self.message, self.location
        )?;

        for note in &self.notes {
            writeln!(f, "\x1b[45m \x1b[1m  NOTE  \x1b[0m {}", note)?;
        }

        Ok(())
    }
}

impl Debug for Panic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Panic! {} at {}", self.message, self.location)?;

        for note in &self.notes {
            write!(f, ". Note: {}", note)?;
        }

        Ok(())
    }
}
