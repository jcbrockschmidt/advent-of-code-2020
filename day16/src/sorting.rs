//! Provides a utility for sorting tickets and fields.

use std::collections::{HashMap, HashSet};

use crate::{Ticket, TicketFieldRule};

/// Finds the field ordering for a set of tickets and a set of unordered field rules.
//
/// Assumes there is exactly one valid field ordering and at least one valid ticket.
pub struct FieldSorter<'a> {
    ticket_rules: Vec<&'a TicketFieldRule>,
    valid_tickets: Vec<&'a Ticket>,
    found_order: bool,
    ordering: Vec<&'a TicketFieldRule>,
    /// Maps a field index to it's possible rules.
    per_field_maybes: Vec<HashSet<&'a TicketFieldRule>>,
    /// Maps a rule to it's possible field indices.
    per_rule_maybes: HashMap<&'a TicketFieldRule, HashSet<usize>>,
}

/// Finds all the valid tickets and calculates the error rate.
pub fn get_valid_tickets<'a>(
    ticket_rules: &Vec<TicketFieldRule>,
    tickets: &'a Vec<Ticket>,
) -> (Vec<&'a Ticket>, u32) {
    let mut valid = Vec::new();
    let mut err_rate = 0;
    for ticket in tickets.iter() {
        let mut is_valid_ticket = true;
        for v in ticket.field_iter() {
            let mut is_valid = false;
            for rule in ticket_rules {
                if rule.check_value(*v) {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                err_rate += v;
                is_valid_ticket = false;
            }
        }
        if is_valid_ticket {
            valid.push(ticket);
        }
    }
    (valid, err_rate)
}

impl<'a> FieldSorter<'a> {
    pub fn new(
        ticket_rules: &'a Vec<TicketFieldRule>,
        valid_tickets: &'a Vec<&'a Ticket>,
    ) -> FieldSorter<'a> {
        assert!(!valid_tickets.is_empty(), "no tickets provided");
        let mut per_field_maybes = Vec::new();
        let mut per_rule_maybes = HashMap::new();
        for rule1 in ticket_rules.iter() {
            let mut possible_rules: HashSet<&TicketFieldRule> = HashSet::new();
            let mut possible_fields: HashSet<usize> = HashSet::new();
            for (i2, rule2) in ticket_rules.iter().enumerate() {
                possible_rules.insert(rule2);
                possible_fields.insert(i2);
            }
            per_field_maybes.push(possible_rules);
            per_rule_maybes.insert(rule1, possible_fields);
        }
        assert!(per_field_maybes.len() == per_rule_maybes.len());
        assert!(per_field_maybes[0].len() == per_field_maybes.len());
        assert!(per_field_maybes[0].len() == per_rule_maybes.iter().next().unwrap().1.len());
        Self {
            ticket_rules: ticket_rules.iter().collect(),
            valid_tickets: valid_tickets.iter().cloned().collect(),
            found_order: false,
            ordering: Vec::new(),
            per_field_maybes: per_field_maybes,
            per_rule_maybes: per_rule_maybes,
        }
    }

    /// Determine that a field cannot be defined by a rule.
    fn remove_relation(&mut self, field_i: usize, rule: &'a TicketFieldRule) -> bool {
        let possible_rules = &mut self.per_field_maybes[field_i];
        let possible_fields = &mut self.per_rule_maybes.get_mut(rule).unwrap();
        possible_rules.remove(rule) | possible_fields.remove(&field_i)
    }

    /// Updates relations so a field is marked as valid only for the rule `valid_rule`.
    fn field_valid_only_for(&mut self, valid_rule: &'a TicketFieldRule) -> bool {
        let mut change = false;
        assert!(self.per_rule_maybes[valid_rule].len() == 1);
        let field_i = *self.per_rule_maybes[valid_rule].iter().next().unwrap();
        // Not using an iter over `self.ticket_rules` to avoid a second mutable borrow.
        for i in 0..self.ticket_rules.len() {
            let rule = self.ticket_rules[i];
            if *rule != *valid_rule {
                change |= self.remove_relation(field_i, rule);
            }
        }
        change
    }

    /// Updates relations so a rule is marked as valid only for the field index `valid_field`.
    fn rule_valid_only_for(&mut self, valid_field: usize) -> bool {
        let mut change = false;
        assert!(self.per_field_maybes[valid_field].len() == 1);
        let rule = *self.per_field_maybes[valid_field].iter().next().unwrap();
        for i in 0..self.per_field_maybes.len() {
            if i != valid_field {
                change |= self.remove_relation(i, rule);
            }
        }
        change
    }

    /// Prunes redundant relations.
    ///
    /// Returns whether a single ordering has been found.
    fn prune(&mut self) -> bool {
        let mut no_variance = true;
        let mut changes = true;
        while changes {
            no_variance = true;
            changes = false;
            for i in 0..self.per_field_maybes.len() {
                if self.per_field_maybes[i].len() == 1 {
                    changes |= self.rule_valid_only_for(i);
                } else {
                    no_variance = false;
                }

                let rule = self.ticket_rules[i];
                if self.per_rule_maybes[rule].len() == 1 {
                    changes |= self.field_valid_only_for(rule);
                } else {
                    no_variance = false;
                }
            }
        }
        no_variance
    }

    /// Finds the ordering of fields using valid tickets found by `get_valid_tickets`.
    pub fn get_field_ordering<'b>(&'b mut self) -> Vec<&'b TicketFieldRule> {
        if self.found_order {
            return self.ordering.iter().cloned().collect();
        }
        let valid_tickets: Vec<&Ticket> = self.valid_tickets.iter().map(|t| *t).collect();
        let mut ordering_found = false;
        for ticket in valid_tickets.iter() {
            for (field_i, v) in ticket.field_iter().enumerate() {
                let possible_rules = &mut self.per_field_maybes[field_i];
                let mut to_remove = Vec::new();
                for rule in possible_rules.iter() {
                    if !rule.check_value(*v) {
                        to_remove.push(*rule);
                    }
                }
                for rule in to_remove.iter() {
                    possible_rules.remove(*rule);
                }
                continue;
            }
            if self.prune() {
                ordering_found = true;
                break;
            }
        }
        assert!(ordering_found);

        // Assumes an ordering has been found by this point.
        for _ in 0..self.ticket_rules.len() {
            self.ordering.push(self.ticket_rules[0]);
        }
        for (rule, possible_fields) in self.per_rule_maybes.iter() {
            // All `HashSet`s should have a length of 1.
            let field_i = *possible_fields.iter().next().unwrap();
            self.ordering[field_i] = rule;
        }

        self.ordering.iter().cloned().collect()
    }
}
