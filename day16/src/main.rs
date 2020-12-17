#[macro_use]
extern crate utils;

use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;
use std::ops::Range;

/// Defines rules for a ticket field to be valid.
struct TicketFieldRule {
    name: String,
    r1: Range<u32>,
    r2: Range<u32>,
}

/// A flight ticket.
struct Ticket {
    values: Vec<u32>,
}

impl TicketFieldRule {
    pub fn new(name: String, range1: Range<u32>, range2: Range<u32>) -> Self {
        Self {
            name: name,
            r1: range1,
            r2: range2,
        }
    }

    /// Checks whether a value if valid for this rule.
    pub fn is_valid(&self, v: u32) -> bool {
        self.r1.contains(&v) || self.r2.contains(&v)
    }
}

impl Ticket {
    pub fn new(values: Vec<u32>) -> Self {
        Self {
            values: values,
        }
    }
}

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Parses a string into a range.
fn parse_range(s: &str) -> Result<Range<u32>, ()> {
    let tokens: Vec<&str> = s.split('-').collect();
    if tokens.len() != 2 {
        return Err(());
    }
    let low: u32 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => return Err(()),
    };
    let high: u32 = match tokens[1].parse() {
        Ok(n) => n,
        Err(_) => return Err(()),
    };
    Ok(low..high + 1)
}

/// Reads tickets rules, your ticket, and other tickets from a file.
fn read_ticket_data<P>(filename: P) -> Result<(Vec<TicketFieldRule>, Ticket, Vec<Ticket>), String>
where
    P: AsRef<Path>,
{
    if !filename.as_ref().exists() {
        return Err("File does not exists".into());
    }
    let lines = match read_lines(filename) {
        Ok(l) => l,
        Err(_) => return Err("Failed to read lines from file".into()),
    };

    let mut skip_next = false;
    let mut reading_rules = true;
    let mut reading_your_ticket = false;
    let mut ticket_rules: Vec<TicketFieldRule> = Vec::new();
    let mut your_ticket: Option<Ticket> = None;
    let mut other_tickets: Vec<Ticket> = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };

        if skip_next {
            skip_next = false;
        } else if line.is_empty() {
            if reading_rules {
                reading_rules = false;
                reading_your_ticket = true;
                skip_next = true;
            } else if reading_your_ticket {
                reading_your_ticket = false;
                skip_next = true;
            }
        } else if reading_rules {
            // Read a rule
            let tokens: Vec<&str> = line.split(':').collect();
            if tokens.len() != 2 {
                return Err(format!("Invalid rule on line {}", i + 1));
            }
            let name = String::from(&tokens[0][0..tokens[0].len() - 1]);

            let range_tokens: Vec<&str> = tokens[1].split(' ').collect();
            if range_tokens.len() != 4 {
                return Err(format!("Invalid ranges on line {}", i + 1));
            }
            let r1 = match parse_range(range_tokens[1]) {
                Ok(r) => r,
                Err(_) => return Err(format!("Bad range on line {}", i + 1)),
            };
            let r2 = match parse_range(range_tokens[3]) {
                Ok(r) => r,
                Err(_) => return Err(format!("Bad range on line {}", i + 1)),
            };
            ticket_rules.push(TicketFieldRule::new(name.into(), r1, r2));
        } else {
            // Read a ticket
            let mut values: Vec<u32> = Vec::new();
            for token in line.split(',') {
                match token.parse::<u32>() {
                    Ok(n) => values.push(n),
                    Err(_) => return Err(format!("Failed to parse value on line {}", i + 1)),
                }
            }
            let ticket = Ticket::new(values);

            // Store ticket
            if reading_your_ticket {
                your_ticket = Some(ticket);
            } else {
                other_tickets.push(ticket);
            }
        }
    }
    if reading_rules || reading_your_ticket {
        Err("Missing a section of ticket data".into())
    } else {
        Ok((ticket_rules, your_ticket.unwrap(), other_tickets))
    }
}

/// Calculate the error rate for invalid ticket fields.
fn calc_scan_err_rate(ticket_rules: &Vec<TicketFieldRule>, tickets: &Vec<Ticket>) -> u32 {
    let mut err_rate = 0;
    for ticket in tickets.iter() {
        for v in ticket.values.iter() {
            let mut is_valid = false;
            for rule in ticket_rules {
                if rule.is_valid(*v) {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                err_rate += v;
            }
        }
    }
    err_rate
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let (ticket_rules, your_ticket, other_tickets) = match read_ticket_data(&input_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read ticket data: {}", e);
            process::exit(1);
        }
    };

    timed_section!("Part 1", { calc_scan_err_rate(&ticket_rules, &other_tickets) }, |v| {
        println!("Ticket scanning error rate: {}", v);
    });
}
