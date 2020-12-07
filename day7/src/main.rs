use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

/// Defines what bags types a bag can hold inside it.
struct BagRule {
    pub color: String,
    pub contains: HashMap<String, u32>,
}

impl BagRule {
    pub fn new(color: String) -> BagRule {
        BagRule {
            color: color,
            contains: HashMap::new(),
        }
    }

    /// Sets the amount of a bag type that this bag can hold
    pub fn set_rule(&mut self, color: String, amount: u32) {
        if amount == 0 {
            if self.contains.contains_key(&color) {
                self.contains.remove(&color);
            }
        } else {
            self.contains.insert(color, amount);
        }
    }
}

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads bad rules from a file.
fn read_bag_rules<P>(filename: P) -> Result<Vec<BagRule>, String>
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

    let mut rules: Vec<BagRule> = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.split(' ').collect();
        if tokens.len() < 7 {
            return Err(format!("Not enough tokens on line {}", i + 1));
        }
        let main_color = format!("{} {}", tokens[0], tokens[1]);
        let mut new_rule = BagRule::new(main_color);
        if tokens.len() > 7 {
            let num_rules = (tokens.len() - 4) / 4;
            for rule_i in 0..num_rules {
                let offset = 4 + rule_i * 4;
                let amount: u32 = match tokens[offset].parse() {
                    Ok(n) => n,
                    Err(_) => return Err(format!("Failed to parse bag amoung on line {}", i + 1)),
                };
                let rule_color = format!("{} {}", tokens[offset + 1], tokens[offset + 2]);
                new_rule.set_rule(rule_color, amount);
            }
            rules.push(new_rule);
        }
    }
    Ok(rules)
}

/// Counts how many bags can contain at least one bag of color `color`.
fn count_contain_color(bag_rules: &Vec<BagRule>, color: String) -> u32 {
    let mut count = 0;
    // Contruct a direct, unweighted graph.
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    for rule in bag_rules {
        // Ignore empty bags. They are irrelevant.
        if rule.contains.len() == 0 {
            continue;
        }
        for contains_color in rule.contains.keys() {
            if !adj_list.contains_key(contains_color) {
                let new_list: Vec<String> = Vec::new();
                adj_list.insert(contains_color.clone(), new_list);
            }
            adj_list
                .get_mut(contains_color)
                .unwrap()
                .push(rule.color.clone());
        }
    }

    // Count all primary and child nodes for `color` using BFS.
    // Assumes there are no loops.
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = match adj_list.get(&color) {
        Some(children) => children.iter().cloned().collect(),
        None => return 0,
    };
    while queue.len() > 0 {
        let color = queue.remove(0);
        if visited.contains(&color) {
            continue;
        }
        visited.insert(color.clone());
        count += 1;
        if let Some(children) = adj_list.get(&color) {
            for c in children {
                queue.push(c.clone());
            }
        }
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
    let bag_rules = match read_bag_rules(&input_path) {
        Ok(answers) => answers,
        Err(e) => {
            eprintln!("Failed to read group answers: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let num_contain_shiny_gold = count_contain_color(&bag_rules, "shiny gold".to_string());
    println!(
        "Number of bags that contain a shiny hold bag: {}",
        num_contain_shiny_gold
    );
}
