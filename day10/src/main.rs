use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
}

/// Reads bad rules from a file.
fn read_adapters<P>(filename: P) -> Result<Vec<u32>, String>
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

    let mut adapters = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        match line.parse() {
            Ok(n) => adapters.push(n),
            Err(_) => return Err(format!("Failed to parse number on line {}", i + 1)),
        }
    }
    adapters.sort();
    Ok(adapters)
}

/// Counts the number of 1-jolt differences and 3-jolt differences when chaining every
/// adapter together.
fn count_jolt_diffs(adapters: &Vec<u32>) -> (u32, u32) {
    if adapters.is_empty() {
        return (0, 0);
    }
    let mut diff1_cnt = 0;
    let mut diff3_cnt = 0;
    // Assume a base joltage of 0.
    let mut prev_jolt = 0;
    for jolt in adapters.iter() {
        let diff = jolt - prev_jolt;
        if diff == 1 {
            diff1_cnt += 1;
        } else if diff == 3 {
            diff3_cnt += 1;
        }
        prev_jolt = *jolt;
    }
    // The final joltage rating is always +3 of the highest adapter.
    diff3_cnt += 1;
    (diff1_cnt, diff3_cnt)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let adapters = match read_adapters(&input_path) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to read adapters: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let (diff1, diff3) = count_jolt_diffs(&adapters);
    println!(
        "{} 1-jolt differences and {} 3-jolt differences",
        diff1, diff3
    );
    println!("Their product is {}", diff1 * diff3);
}
