//! Provides a utility for sorting tickets and fields.

use crate::{Ticket, TicketFieldRule};

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
