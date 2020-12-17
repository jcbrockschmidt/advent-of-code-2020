#[macro_use]
extern crate utils;

use std::env;
use std::process;

use day16::sorting;
use day16::ticket_parser;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
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
        { sorting::get_valid_tickets(&ticket_rules, &other_tickets) },
        |(valid, err_rate)| {
            println!("Ticket scanning error rate: {}", err_rate);
            valid
        }
    );
}
