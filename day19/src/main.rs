#[macro_use]
extern crate utils;

mod message_rules;

use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

use message_rules::{MessageRule, MessageRules};

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads message rules from a file.
fn read_message_rules<P>(filename: P) -> Result<(MessageRules, Vec<String>), String>
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

    let mut rules = MessageRules::new();
    let mut strings: Vec<String> = Vec::new();
    let mut reading_rules = true;
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            if reading_rules {
                reading_rules = false;
            }
        } else if reading_rules {
            // Read a rule.
            let tokens: Vec<&str> = line.split(' ').collect();
            let rule_i_str = &tokens[0][0..tokens[0].len() - 1];
            let rule_i: usize = match rule_i_str.parse() {
                Ok(n) => n,
                Err(_) => return Err(format!("Failed to parse number on line {}", i + 1)),
            };
            let rule_nums = {
                let mut nums: Vec<usize> = Vec::new();
                for token in tokens.iter().skip(1) {
                    if let Ok(n) = token.parse::<usize>() {
                        nums.push(n);
                    }
                }
                nums
            };
            let new_rule = match tokens.len() {
                2 => {
                    let chars: Vec<char> = tokens[1].to_string().chars().collect();
                    if chars[0] == '"' {
                        MessageRule::Char(chars[1])
                    } else if rule_nums.len() == 1 {
                        MessageRule::OtherRule(rule_nums[0])
                    } else {
                        return Err(format!("Bad rule on line {}", i + 1));
                    }
                }
                3 => {
                    if rule_nums.len() == 2 {
                        MessageRule::And(rule_nums[0], rule_nums[1])
                    } else {
                        return Err(format!("Bad rule on line {}", i + 1));
                    }
                }
                4 => {
                    if tokens[2] == "|" && rule_nums.len() == 2 {
                        MessageRule::Or(rule_nums[0], rule_nums[1])
                    } else if rule_nums.len() == 3 {
                        MessageRule::And3(rule_nums[0], rule_nums[1], rule_nums[2])
                    } else {
                        return Err(format!("Bad rule on line {}", i + 1));
                    }
                }
                6 => {
                    if rule_nums.len() == 4 {
                        MessageRule::AndOrAnd(
                            (rule_nums[0], rule_nums[1]),
                            (rule_nums[2], rule_nums[3]),
                        )
                    } else {
                        return Err(format!("Bad rule on line {}", i + 1));
                    }
                }
                _ => return Err(format!("Invalid number of tokens on line {}", i + 1)),
            };
            rules.add_rule(rule_i, new_rule);
        } else {
            // Read a string.
            strings.push(line);
        }
    }
    Ok((rules, strings))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let (rules, strings) =
        timed_section!("Initialization", { read_message_rules(&input_path) }, |v| {
            match v {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to read message rules and strings: {}", e);
                    process::exit(1);
                }
            }
        });

    timed_section!(
        "Part 1",
        {
            let mut num_valid: usize = 0;
            for s in strings {
                if rules.check_string(s) {
                    num_valid += 1;
                }
            }
            num_valid
        },
        |num_valid| {
            println!("Number of valid strings: {}", num_valid);
        }
    );
}
