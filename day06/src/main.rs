use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads "yes" answers from a file and groups them.
fn read_group_answers<P>(filename: P) -> Result<Vec<Vec<HashSet<char>>>, String>
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

    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    let mut cur_group: Vec<HashSet<char>> = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            groups.push(cur_group);
            cur_group = Vec::new();
        } else {
            let answers: HashSet<char> = line.chars().collect();
            cur_group.push(answers);
        }
    }
    if !cur_group.is_empty() {
        groups.push(cur_group);
    }
    Ok(groups)
}

/// Sums total number of answers per-group where someone answered yes.
fn sum_group_answers_union(group_answers: &Vec<Vec<HashSet<char>>>) -> usize {
    let mut total = 0;
    for group in group_answers {
        let agg: HashSet<char> = group
            .iter()
            .fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect());
        total += agg.len();
    }
    total
}

/// Sums total number of answers per-group where everyone answered yes.
fn sum_group_answers_inter(group_answers: &Vec<Vec<HashSet<char>>>) -> usize {
    let mut total = 0;
    for group in group_answers {
        let first = group[0].iter().cloned().collect();
        let agg: HashSet<char> = group
            .iter()
            .skip(1)
            .fold(first, |acc, x| acc.intersection(&x).cloned().collect());
        total += agg.len();
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let group_answers = match read_group_answers(&input_path) {
        Ok(answers) => answers,
        Err(e) => {
            eprintln!("Failed to read group answers: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let union_sum = sum_group_answers_union(&group_answers);
    println!("Total \"yes\" answers, aggregated by union: {}", union_sum);

    println!("\n==== Part 2 ====");
    let inter_sum = sum_group_answers_inter(&group_answers);
    println!(
        "Total \"yes\" answers, aggregated by intersection: {}",
        inter_sum
    );
}
