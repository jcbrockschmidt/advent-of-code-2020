#[macro_use]
extern crate lazy_static;

mod passport;

use std::path::Path;
use std::{env, process};
use utils::read_lines;

use passport::Passport;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads passports from a file.
fn read_passports<P>(filename: P) -> Result<Vec<Passport>, String>
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
    let mut passports: Vec<Passport> = Vec::new();
    let mut cur_pp = Passport::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.len() == 0 {
            passports.push(cur_pp);
            cur_pp = Passport::new();
        } else {
            for token in line.split(" ") {
                let parts: Vec<&str> = token.split(":").collect();
                if parts.len() < 2 {
                    continue;
                }
                let value = parts[1].to_string();
                match parts[0] {
                    "byr" => {
                        cur_pp.byr = value;
                    }
                    "iyr" => {
                        cur_pp.iyr = value;
                    }
                    "eyr" => {
                        cur_pp.eyr = value;
                    }
                    "hgt" => {
                        cur_pp.hgt = value;
                    }
                    "hcl" => {
                        cur_pp.hcl = value;
                    }
                    "ecl" => {
                        cur_pp.ecl = value;
                    }
                    "pid" => {
                        cur_pp.pid = value;
                    }
                    "cid" => {
                        cur_pp.cid = value;
                    }
                    _ => {}
                }
            }
        }
    }
    passports.push(cur_pp);

    Ok(passports)
}

/// Counts the number of passports that have all the required fields.
fn count_valid_passports_lazy(passports: &Vec<Passport>) -> i32 {
    let mut num_valid: i32 = 0;
    for pp in passports.iter() {
        if pp.has_required_fields() {
            num_valid += 1;
        }
    }
    num_valid
}

/// Counts the number of passports that have valid values in all required fields.
fn count_valid_passports_strict(passports: &Vec<Passport>) -> i32 {
    let mut num_valid: i32 = 0;
    for pp in passports.iter() {
        if pp.has_valid_values() {
            num_valid += 1;
        }
    }
    num_valid
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let passports = match read_passports(&input_path) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read passports: {}", e);
            process::exit(1);
        }
    };
    println!("{} passports found (none verified yet)", passports.len());

    println!("\n==== Part 1 ====");
    let num_valid_lazy = count_valid_passports_lazy(&passports);
    println!("{} valid passports", num_valid_lazy);

    println!("\n==== Part 2 ====");
    let num_valid_strict = count_valid_passports_strict(&passports);
    println!("{} valid passports", num_valid_strict);
}
