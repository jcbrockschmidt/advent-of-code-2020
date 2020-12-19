#[macro_use]
extern crate utils;

mod equation;

use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

use equation::Equation;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads equations from a file.
fn read_equations<P>(filename: P) -> Result<Vec<Equation>, String>
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

    let mut eqs = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        match Equation::new(line) {
            Ok(eq) => eqs.push(eq),
            Err(e) => return Err(format!("Failed to read equation on line {}: {}", i + 1, e)),
        }
    }
    Ok(eqs)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);

    let equations = timed_section!(
        "Initialization",
        { read_equations(&input_path) },
        |res: Result<Vec<Equation>, String>| {
            match res {
                Ok(eqs) => {
                    println!("Read {} equations", eqs.len());
                    eqs
                }
                Err(e) => {
                    eprintln!("Failed to read equations: {}", e);
                    process::exit(1);
                }
            }
        }
    );

    timed_section!(
        "Part 1",
        {
            let mut sum: u64 = 0;
            for eq in equations.iter() {
                sum += eq.eval();
            }
            sum
        },
        |v| {
            println!("Sum of all equation results: {}", v);
        }
    );
}
