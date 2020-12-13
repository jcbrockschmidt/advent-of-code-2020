use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
}

/// Reads departure time and bus IDs from a file.
fn read_bus_data<P>(filename: P) -> Result<(u32, Vec<u32>), String>
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

    let mut lines_str = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        lines_str.push(line);
    }
    if lines_str.len() < 2 {
        return Err(format!("Not enough lines"));
    }
    let depart = match lines_str[0].parse() {
        Ok(n) => n,
        Err(_) => {
            return Err(format!(
                "Failed to parse departure time \"{}\"",
                lines_str[0]
            ))
        }
    };
    let mut bus_ids = Vec::new();
    for id_str in lines_str[1].split(',') {
        if id_str == "x" {
            continue;
        }
        match id_str.parse::<u32>() {
            Ok(id) => bus_ids.push(id),
            Err(_) => return Err(format!("Failed to parse bus ID \"{}\"", id_str)),
        }
    }
    Ok((depart, bus_ids))
}

/// Calculates the time until a bus arrives after a given `start` time.
fn get_time_until_bus(start: u32, bus_id: u32) -> u32 {
    let time = start / bus_id * bus_id;
    if time == start {
        time - start
    } else {
        time + bus_id - start
    }
}

/// Gets the bus with the shortest time until arrival after a given `start` time.
/// Returns the bus ID and minimum wait time.
fn get_earliest_bus(start: u32, bus_ids: &Vec<u32>) -> Option<(u32, u32)> {
    if bus_ids.len() == 0 {
        return None;
    }
    let mut best_bus_id = bus_ids[0];
    let mut min_wait_time = get_time_until_bus(start, best_bus_id);
    for bus_id in bus_ids.iter().skip(1) {
        let wait_time = get_time_until_bus(start, *bus_id);
        if wait_time < min_wait_time {
            best_bus_id = *bus_id;
            min_wait_time = wait_time;
        }
    }
    return Some((best_bus_id, min_wait_time));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let (depart, bus_ids) = match read_bus_data(&input_path) {
        Ok((d, bids)) => (d, bids),
        Err(e) => {
            eprintln!("Failed to read bus data: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    match get_earliest_bus(depart, &bus_ids) {
        Some((earliest_bus, wait_time)) => {
            println!("Earliest available bus: {}", earliest_bus);
            println!("Wait time: {}", wait_time);
            println!(
                "Product of bus ID and wait time: {}",
                earliest_bus * wait_time
            );
        }
        None => {
            println!("No buses found");
        }
    }
}
