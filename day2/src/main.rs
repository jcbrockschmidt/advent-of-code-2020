use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Gets an iterator for the lines in a file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Checks a password's validity according to a policy.
fn check_pwd_validity(ch: char, min_occurs: i32, max_occurs: i32, passwd: String) -> bool {
    //println!("{} {} {}, {}", min_occurs, max_occurs, ch, passwd);
    let mut occurs = 0;
    for c in passwd.chars() {
        if c == ch {
            occurs += 1;
            if occurs > max_occurs {
                return false;
            }
        }
    }
    occurs >= min_occurs
}

/// Returns the number of valid passwords in a file.
fn check_policies<P>(filename: P) -> Result<i32, String>
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

    let mut num_valid = 0;
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err("Failed to read a line".into()),
        };
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens.len() != 3 {
            return Err(format!("Not enough tokens on line {}", i + 1));
        }

        let bounds: Vec<&str> = tokens[0].split("-").collect();
        if bounds.len() != 2 {
            return Err(format!("Invalid bounds on line {}", i + 1));
        }
        let min_occurs: i32 = match bounds[0].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!(
                    "Invalid bound {} for policy on line {}",
                    bounds[0],
                    i + 1
                ))
            }
        };
        let max_occurs: i32 = match bounds[1].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!(
                    "Invalid bound {} for policy on line {}",
                    bounds[1],
                    i + 1
                ))
            }
        };

        let ch = match tokens[1].chars().nth(0) {
            Some(c) => c,
            None => return Err(format!("No character found for policy on line {}", i + 1)),
        };

        let passwd: String = tokens[2].into();

        if check_pwd_validity(ch, min_occurs, max_occurs, passwd) {
            num_valid += 1;
        }
    }
    Ok(num_valid)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = Path::new(&args[1]);

    // Part 1
    println!("==== Part 1 ====");
    match check_policies(input_path) {
        Ok(num_valid) => {
            println!("{} valid policies found", num_valid);
        }
        Err(e) => {
            eprintln!("Part 1 failed: {}", e);
            process::exit(1);
        }
    }
}
