//! Provides a rule for a ticket field.

use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::ops::Range;

/// Defines rules for a ticket field, used to determine validity of a field.
#[derive(Clone)]
pub struct TicketFieldRule {
    name: String,
    r1: Range<u32>,
    r2: Range<u32>,
}

impl TicketFieldRule {
    pub fn new(name: String, range1: Range<u32>, range2: Range<u32>) -> Self {
        Self {
            name: name,
            r1: range1,
            r2: range2,
        }
    }

    /// Gets the name of a field.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Checks whether a value is valid for this rule.
    pub fn check_value(&self, v: u32) -> bool {
        self.r1.contains(&v) || self.r2.contains(&v)
    }
}

impl PartialEq for TicketFieldRule {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r1 == other.r1 && self.r2 == other.r2
    }
}

impl Eq for TicketFieldRule {}

impl Hash for TicketFieldRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.r1.hash(state);
        self.r2.hash(state);
    }
}
