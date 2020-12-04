#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::i64;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

lazy_static! {
    static ref VALID_EYE_COLORS: Vec<String> = {
        let mut vec: Vec<String> = Vec::new();
        vec.push("amb".into());
        vec.push("blu".into());
        vec.push("brn".into());
        vec.push("gry".into());
        vec.push("grn".into());
        vec.push("hzl".into());
        vec.push("oth".into());
        vec
    };
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

struct Passport {
    pub byr: String,
    pub iyr: String,
    pub eyr: String,
    pub hgt: String,
    pub hcl: String,
    pub ecl: String,
    pub pid: String,
    pub cid: String,
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            byr: "".into(),
            iyr: "".into(),
            eyr: "".into(),
            hgt: "".into(),
            hcl: "".into(),
            ecl: "".into(),
            pid: "".into(),
            cid: "".into(),
        }
    }

    /// Checks that all required fields have a non-empty value.
    pub fn check_has_required(&self) -> bool {
        self.byr.len() > 0
            && self.iyr.len() > 0
            && self.eyr.len() > 0
            && self.hgt.len() > 0
            && self.hcl.len() > 0
            && self.ecl.len() > 0
            && self.pid.len() > 0
    }

    /// Checks that all required fields have valid values.
    pub fn check_valid_values(&self) -> bool {
        if !self.check_has_required() {
            return false;
        }
        // Birth year
        match self.byr.parse::<u16>() {
            Ok(n) => {
                if n < 1920 || n > 2002 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        // Issue year
        match self.iyr.parse::<u16>() {
            Ok(n) => {
                if n < 2010 || n > 2020 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        // Expiration year
        match self.eyr.parse::<u16>() {
            Ok(n) => {
                if n < 2020 || n > 2030 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        // Height
        if self.hgt.len() < 3 {
            return false;
        }
        let hgt_len = self.hgt.len();
        let hgt_val = match self.hgt[0..hgt_len - 2].parse::<u16>() {
            Ok(n) => n,
            Err(_) => {
                println!("parse fail"); // DEBUG
                return false;
            }
        };
        let hgt_unit = &self.hgt[hgt_len - 2..hgt_len];
        match hgt_unit {
            "in" => {
                if hgt_val < 59 || hgt_val > 76 {
                    return false;
                }
            }
            "cm" => {
                if hgt_val < 150 || hgt_val > 193 {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }

        // Hair color
        if self.hcl.len() != 7 {
            return false;
        }
        if self.hcl.chars().collect::<Vec<char>>()[0] != '#' {
            return false;
        }
        if i64::from_str_radix(&self.hcl[1..], 16).is_err() {
            return false;
        }

        // Eye color
        if !VALID_EYE_COLORS.contains(&self.ecl) {
            return false;
        }

        // Passport ID
        if self.pid.len() != 9 {
            return false;
        }
        if !self.pid.chars().all(char::is_numeric) {
            return false;
        }

        true
    }
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

fn count_valid_passports_strict(passports: &Vec<Passport>) -> i32 {
    let mut num_valid: i32 = 0;
    for pp in passports.iter() {
        if pp.check_valid_values() {
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
    let input_path = Path::new(&args[1]);
    let passports = match read_passports(&input_path) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read passports: {}", e);
            process::exit(1);
        }
    };
    println!("{} passports found (none unverified yet)", passports.len());

    println!("\n==== Part 1 ====");
    let num_valid = match count_valid_passports(&input_path) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to count valid passports: {}", e);
            process::exit(1);
        }
    };
    println!("{} valid passports", num_valid);

    println!("\n==== Part 2 ====");
    let num_valid_strict = count_valid_passports_strict(&passports);
    println!("{} valid passports", num_valid_strict);
}
