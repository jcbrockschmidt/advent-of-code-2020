#[macro_use]
extern crate utils;

use std::env;
use std::process;

use day16::sorting::{get_valid_tickets, FieldSorter};
use day16::{ticket_parser, Ticket, TicketFieldRule};

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Calculates the product of all fields starting with "departure" for a ticket.
fn mult_depart_values(ticket: &Ticket, ordered_fields: &Vec<&TicketFieldRule>) -> usize {
    let mut prod: usize = 1;
    for (i, v) in ticket.field_iter().enumerate() {
        let rule = ordered_fields[i];
        if rule.get_name().starts_with("departure") {
            prod *= *v as usize;
        }
    }
    prod
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let (ticket_rules, your_ticket, other_tickets) =
        match ticket_parser::read_ticket_data(&input_path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to read ticket data: {}", e);
                process::exit(1);
            }
        };

    let valid_tickets = timed_section!(
        "Part 1",
        { get_valid_tickets(&ticket_rules, &other_tickets) },
        |(valid, err_rate)| {
            println!("Ticket scanning error rate: {}", err_rate);
            valid
        }
    );

    timed_section!(
        "Part 2",
        {
            let mut sorter = FieldSorter::new(&ticket_rules, &valid_tickets);
            let ordered_fields = sorter.get_field_ordering();
            mult_depart_values(&your_ticket, &ordered_fields)
        },
        |v| {
            println!("Product of departure values: {}", v);
        }
    );
}
