use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
}

/// Reads numbers from a file.
fn read_numbers<P>(filename: P) -> Result<Vec<usize>, String>
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

    let mut nums = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        match line.parse::<usize>() {
            Ok(n) => nums.push(n),
            Err(_) => return Err(format!("Failed to parse number on line {}", i + 1)),
        };
    }
    Ok(nums)
}

/// Finds the first number that breaks the XMAS encryption.
fn find_invalid_num(nums: &Vec<usize>, scope: usize) -> Option<usize> {
    // First `scope` numbers are always valid.
    if nums.len() <= scope {
        return None;
    }
    // Finds the first number in the list that is not a sum of any two of the preceding `scope` numbers.
    for (i, n1) in nums[scope..].iter().enumerate() {
        let mut is_valid = false;
        let prev_nums = &nums[i..i + scope];
        for n2 in prev_nums.iter() {
            if *n2 > *n1 {
                continue;
            }
            for n3 in prev_nums.iter() {
                if *n2 + *n3 == *n1 {
                    is_valid = true;
                    break;
                }
            }
            if is_valid {
                break;
            }
        }
        if !is_valid {
            return Some(*n1);
        }
    }
    None
}

/// Finds the encryption weakness in the XMAS-encrypted list of numbers given an invalid number `invalid_num` yielded by `find_invalid_num`.
fn find_weakness(nums: &Vec<usize>, invalid_num: usize) -> Option<usize> {
    // Find a contiguous set of numbers that equals our invalid number.
    for (i, n1) in nums[..nums.len() - 1].iter().enumerate() {
        let mut min = *n1;
        let mut max = *n1;
        let mut sum = *n1;
        for n2 in nums[i + 1..].iter() {
            sum += *n2;
            if *n2 < min {
                min = *n2;
            }
            if *n2 > max {
                max = *n2
            }
            if sum == invalid_num {
                // Encryption weakness found.
                return Some(min + max);
            } else if sum > invalid_num {
                break;
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let scope: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("scope must be an unsigned integer\n");
            process::exit(1);
        }
    };
    let nums = match read_numbers(input_path) {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("Failed to read numbers from file: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let invalid_num = match find_invalid_num(&nums, scope) {
        Some(n) => n,
        None => {
            eprintln!("No invalid number found");
            process::exit(1);
        }
    };
    println!("First invalid number: {}", invalid_num);

    println!("\n==== Part 2 ====");
    let weakness = match find_weakness(&nums, invalid_num) {
        Some(n) => n,
        None => {
            eprintln!("Failed to find encryption weakness");
            process::exit(1);
        }
    };
    println!("Encryption weakness: {}", weakness);
}
