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

/// Finds two numbers in sorted `numbers` that sum up to `sum`.
fn find_sum_of_two(numbers: &Vec<i64>, sum: i64) -> Option<(i64, i64)> {
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

/// Finds three numbers in the sorted `numbers` that sum up to `sum`.
fn find_sum_of_three(numbers: &Vec<i64>, sum: i64) -> Option<(i64, i64, i64)> {
    // Precompute the sums of all numbers where their sum < sum
    let mut pre_sums: Vec<(i64, i64, i64)> = Vec::new();
    for (i, n1) in numbers.iter().enumerate() {
        let inner_iter = numbers.iter().skip(i + 1);
        for n2 in inner_iter {
            let n_sum = n1 + n2;
            if n_sum >= sum {
                break;
            } else {
                let v = (n_sum, *n1, *n2);
                // Insert sum in ascending sorted order.
                let mut insert_at = pre_sums.len();
                for (i, (sum, _, _)) in pre_sums.iter().enumerate() {
                    if &n_sum < sum {
                        insert_at = i;
                        break;
                    }
                }
                pre_sums.insert(insert_at, v);
            }
        }
    }

    for n3 in numbers {
        for (pre_sum, n1, n2) in &pre_sums {
            let n_sum = n3 + pre_sum;
            if n_sum == sum {
                return Some((*n1, *n2, *n3));
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

    // Part 1
    match find_sum_of_two(&numbers, DESIRED_SUM) {
        Some((n1, n2)) => {
            println!("{} and {} sum to {}", n1, n2, DESIRED_SUM);
            println!("{} * {} = {}", n1, n2, n1 * n2);
        }
        None => println!("No two numbers found that sum to {}", DESIRED_SUM),
    }

    // Part 2
    match find_sum_of_three(&numbers, DESIRED_SUM) {
        Some((n1, n2, n3)) => {
            println!("{}, {}, {} sum to {}", n1, n2, n3, DESIRED_SUM);
            println!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3);
        }
        None => println!("No three numbers found that sum to {}", DESIRED_SUM),
    }
}
