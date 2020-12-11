mod seating_map;

use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

use seating_map::SeatingMap;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
}

/// Reads bad rules from a file.
fn read_seating_map<P>(filename: P) -> Result<SeatingMap, String>
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

    let mut chars: Vec<Vec<char>> = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        chars.push(line.chars().collect());
    }
    if chars.len() == 0 {
        return Ok(SeatingMap::new(0, 0));
    }
    let mut map = SeatingMap::new(chars[0].len(), chars.len());
    for (y, row) in chars.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'L' {
                map.add_seat(x, y);
            }
        }
    }
    Ok(map)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let seating_map = match read_seating_map(&input_path) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Failed to read seating map: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let stable_adj = seating_map.get_stable_adj();
    let filled_adj = stable_adj.count_filled_seats();
    println!("Final number of filled seats: {}", filled_adj);

    println!("\n==== Part 2 ====");
    let stable_sight = seating_map.get_stable_sightline();
    let filled_sight = stable_sight.count_filled_seats();
    println!("Final number of filled seats: {}", filled_sight);
}
