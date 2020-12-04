#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

lazy_static! {
    static ref REQUIRED_FIELDS: Vec<String> = {
        let mut vec: Vec<String> = Vec::new();
        vec.push("byr".into());
        vec.push("iyr".into());
        vec.push("eyr".into());
        vec.push("hgt".into());
        vec.push("hcl".into());
        vec.push("ecl".into());
        vec.push("pid".into());
        vec
    };
}

/// Prints usage statement for the executable.
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

/// Counts the number of valid passports.
fn count_valid_passports<P>(filename: P) -> Result<i32, String>
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
    // Assumes there are no duplicate fields.
    let mut num_valid: i32 = 0;
    let total_fields = REQUIRED_FIELDS.len();
    let mut req_field_cnt = 0;
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.len() == 0 {
            if req_field_cnt == total_fields {
                num_valid += 1;
            }
            req_field_cnt = 0;
        } else {
            for token in line.split(" ") {
                let parts: Vec<&str> = token.split(":").collect();
                if parts.len() < 2 {
                    return Err(format!("Invalid field-value pair on line {}", i + 1));
                }
                if REQUIRED_FIELDS.contains(&parts[0].into()) {
                    req_field_cnt += 1;
                }
            }
        }
    }
    if req_field_cnt == total_fields {
        num_valid += 1;
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

    println!("==== Part 1 ====");
    let num_valid = match count_valid_passports(&input_path) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to count valid passports: {}", e);
            process::exit(1);
        }
    };
    println!("{} valid passports", num_valid);
}
