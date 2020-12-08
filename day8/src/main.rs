use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

enum InstrType {
    ACC,
    JMP,
    NOP,
}

struct Instruction {
    pub typ: InstrType,
    pub val: i64,
}

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads bad rules from a file.
fn read_instructions<P>(filename: P) -> Result<Vec<Instruction>, String>
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

    let mut ins = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.split(' ').collect();
        if tokens.len() < 2 {
            return Err(format!("Not enough tokens on line {}", i + 1));
        }
        let typ = match tokens[0] {
            "acc" => InstrType::ACC,
            "jmp" => InstrType::JMP,
            "nop" => InstrType::NOP,
            _ => {
                return Err(format!(
                    "Unrecognized instruction \"{}\" on line {}",
                    tokens[0],
                    i + 1
                ))
            }
        };
        let val = match tokens[1].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!(
                    "Failed to parse value \"{}\" on line {}",
                    tokens[1],
                    i + 1
                ))
            }
        };
        ins.push(Instruction { typ: typ, val: val });
    }
    Ok(ins)
}

/// Gets the value of the accumulator as per the instruction set before any instructions repeat.
fn get_acc_before_repeats(ins: &Vec<Instruction>) -> i64 {
    let mut visited = Vec::new();
    for _ in 0..ins.len() {
        visited.push(false);
    }
    let mut i: i64 = 0;
    let mut acc: i64 = 0;
    while i >= 0 && !visited[i as usize] {
        visited[i as usize] = true;
        let cur_in = &ins[i as usize];
        match cur_in.typ {
            InstrType::ACC => {
                acc += cur_in.val;
                i += 1;
            }
            InstrType::JMP => {
                i += cur_in.val;
            }
            InstrType::NOP => {
                i += 1;
            }
        }
    }
    acc
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let ins = match read_instructions(input_path) {
        Ok(ins) => ins,
        Err(e) => {
            eprintln!("Failed to read instructions from file: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let acc_val = get_acc_before_repeats(&ins);
    println!("Accumulator value before repeats: {}", acc_val);
}
