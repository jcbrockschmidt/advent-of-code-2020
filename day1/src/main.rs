use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, process};

const DESIRED_SUM: i64 = 2020;

fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Parses each line of a file for numbers, returning a sorted vector of numbers.
fn parse_file<P>(filename: P) -> Result<Vec<i64>, String>
where
    P: AsRef<Path>,
{
    if !filename.as_ref().exists() {
        return Err("File does not exists".into());
    }
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Err("Could not open file".into()),
    };
    let mut nums: Vec<i64> = Vec::new();
    let lines = BufReader::new(file).lines();
    for line_res in lines {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err("Failed to read a line".into()),
        };
        if line.is_empty() {
            continue;
        }
        let num: i64 = match line.parse() {
            Ok(n) => n,
            Err(_) => return Err(format!("Invalid number \"{}\"", line)),
        };

        // Insert number in sorted order.
        match nums.binary_search(&num) {
            Ok(_) => {} // Number already in `nums`.
            Err(pos) => nums.insert(pos, num),
        }
    }
    Ok(nums)
}

/// Finds two numbers in `numbers` that sum up to `sum`.
fn find_sum(numbers: &Vec<i64>, sum: i64) -> Option<(i64, i64)> {
    for (i, n1) in numbers.iter().enumerate() {
        let inner_iter = numbers.iter().skip(i + 1);
        for n2 in inner_iter {
            let n_sum = n1 + n2;
            if n_sum == sum {
                return Some((*n1, *n2));
            } else if n_sum > sum {
                break;
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let numbers = match parse_file(&input_path) {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("Failed to parse \"{}\": {}", &input_path.display(), e);
            process::exit(1);
        }
    };

    match find_sum(&numbers, DESIRED_SUM) {
        Some((n1, n2)) => {
            println!("{} and {} sum to {}", n1, n2, DESIRED_SUM);
            println!("{} * {} = {}", n1, n2, n1 * n2);
        }
        None => println!("No two numbers found that sum to {}", DESIRED_SUM),
    }
}
