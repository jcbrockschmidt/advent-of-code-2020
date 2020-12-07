use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads answers from a file and aggregates them by group.
fn read_group_answers<P>(filename: P) -> Result<Vec<HashSet<char>>, String>
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

    let mut answer_sets: Vec<HashSet<char>> = Vec::new();
    let mut cur_set = HashSet::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            answer_sets.push(cur_set);
            cur_set = HashSet::new();
        } else {
            for ch in line.chars() {
                cur_set.insert(ch);
            }
        }
    }
    if !cur_set.is_empty() {
        answer_sets.push(cur_set);
    }
    Ok(answer_sets)
}

/// Counts the total number of answers over a collection of answer sets.
fn count_yes_answers(answer_sets: &Vec<HashSet<char>>) -> usize {
    let mut count = 0;
    for answers in answer_sets {
        count += answers.len();
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let group_answers = match read_group_answers(input_path) {
        Ok(answers) => answers,
        Err(e) => {
            eprintln!("Failed to read group answers: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let total_answers = count_yes_answers(&group_answers);
    println!(
        "Total \"yes\" answers, aggregated by group: {}",
        total_answers
    );
}
