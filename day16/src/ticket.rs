//! Provides a ticket with ordered, unlabeled fields.

use std::slice::Iter;

/// A ticket with ordered, unlabeled fields.
#[derive(Clone)]
pub struct Ticket {
    pub values: Vec<u32>,
}

impl Ticket {
    pub fn new(values: Vec<u32>) -> Self {
        Self { values: values }
    }

    /// Returns an iterator over all field values in a ticket.
    pub fn field_iter<'a>(&'a self) -> Iter<'a, u32> {
        self.values.iter()
    }
}
