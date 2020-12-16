#[macro_use]
extern crate utils;

use std::collections::HashMap;
use std::env;
use std::process;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} starting_numbers", args[0]);
}

/// Returns the number spoken on the `last_turn`th turn.
fn memory_game(start_nums: &Vec<usize>, last_turn: usize) -> usize {
    let mut last_spoke: HashMap<usize, usize> = HashMap::new();
    for (i, n) in start_nums.iter().enumerate() {
        if i == start_nums.len() - 1 {
            break;
        }
        // Turn numbers start at 1, not 0.
        last_spoke.insert(*n, i + 1);
    }
    // Assumes there is at least one start number.
    let mut prev = *start_nums.last().unwrap();
    for turn in (start_nums.len() + 1)..(last_turn + 1) {
        let cur: usize = match last_spoke.get(&prev) {
            Some(last) => turn - 1 - last,
            None => 0,
        };
        last_spoke.insert(prev, turn - 1);
        prev = cur;
    }
    prev
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let start_nums: Vec<usize> = {
        let mut nums = Vec::new();
        for num in args[1].split(',') {
            match num.parse::<usize>() {
                Ok(n) => nums.push(n),
                Err(_) => {
                    eprintln!("Failed to parse starting numbers");
                    process::exit(1);
                }
            }
        }
        nums
    };

    timed_section!("Part 1", { memory_game(&start_nums, 2020) }, |v| {
        println!("At turn 2020: {}", v);
    });

    timed_section!("Part 2", { memory_game(&start_nums, 30_000_000) }, |v| {
        println!("At 30000000 turns: {}", v);
    });
}
