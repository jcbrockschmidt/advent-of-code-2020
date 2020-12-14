#[macro_use]
extern crate utils;

use std::env;
use std::path::Path;
use std::process;
use std::usize;
use utils::read_lines;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
}

/// Reads departure time and bus IDs from a file.
fn read_bus_data<P>(filename: P) -> Result<(u32, Vec<Option<u32>>), String>
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
            bus_ids.push(None);
        } else {
            match id_str.parse::<u32>() {
                Ok(id) => bus_ids.push(Some(id)),
                Err(_) => return Err(format!("Failed to parse bus ID \"{}\"", id_str)),
            }
        }
    }
    Ok((depart, bus_ids))
}

/// Calculates the time until a bus arrives after a given `start` time.
fn get_time_until_bus(start: u32, bus_id: u32) -> usize {
    // Two modulos are use to avoid a subtract with underflow.
    ((bus_id - start % bus_id) % bus_id) as usize
}

/// Gets the bus with the shortest time until arrival after a given `start` time.
/// Returns the bus ID and minimum wait time.
fn get_earliest_bus(start: u32, bus_ids: &Vec<Option<u32>>) -> Option<(u32, usize)> {
    let mut best_bus_id: Option<u32> = None;
    let mut min_wait_time: usize = usize::MAX;
    for bus_id in bus_ids.iter() {
        if let Some(id) = bus_id.as_ref() {
            let wait_time = get_time_until_bus(start, *id);
            if wait_time < min_wait_time {
                best_bus_id = Some(*id);
                min_wait_time = wait_time;
            }
        }
    }
    best_bus_id.map(|id| (id, min_wait_time))
}

/// Find the lowest time at which all buses leave in order a minute after the last.
fn get_lowest_subsequent_depart_time(bus_ids: &Vec<Option<u32>>) -> usize {
    let mut step = 1;
    let mut t: usize = 0;
    for cur_id in bus_ids.iter() {
        if let Some(bus_incr) = cur_id.map(|id| id as usize) {
            loop {
                if t % bus_incr == 0 {
                    step *= bus_incr;
                    break;
                } else {
                    t += step;
                }
            }
        }
        t += 1;
    }
    t - bus_ids.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let (depart, bus_ids) = timed_section!("Initialization", { read_bus_data(&input_path) }, |v| {
        match v {
            Ok((d, bids)) => (d, bids),
            Err(e) => {
                eprintln!("Failed to read bus data: {}", e);
                process::exit(1);
            }
        }
    });

    timed_section!("Part 1", { get_earliest_bus(depart, &bus_ids) }, |v| {
        match v {
            Some((earliest_bus, wait_time)) => {
                println!("Earliest available bus: {}", earliest_bus);
                println!("Wait time: {}", wait_time);
                println!(
                    "Product of bus ID and wait time: {}",
                    (earliest_bus as usize) * wait_time
                );
            }
            None => {
                println!("No buses found");
            }
        }
    });

    timed_section!(
        "Part 2",
        { get_lowest_subsequent_depart_time(&bus_ids) },
        |soonest_seq_time| {
            println!(
                "Soonest time for synchronous subsequent departures: {}",
                soonest_seq_time
            );
        }
    );
}
