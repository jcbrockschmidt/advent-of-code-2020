use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
}

/// Reads bad rules from a file.
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

/// Finds the first number in the list that is not a sum of any two of the preceding `scope` numbers.
fn find_invalid_num(nums: &Vec<usize>, scope: usize) -> Option<usize> {
    if nums.len() <= scope {
        return None;
    }
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
    match find_invalid_num(&nums, scope) {
        Some(n) => println!("First invalid number: {}", n),
        None => println!("No invalid number found"),
    }
}
