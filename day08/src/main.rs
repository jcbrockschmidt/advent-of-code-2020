use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

#[derive(Clone)]
enum InstrType {
    ACC,
    JMP,
    NOP,
}

#[derive(Clone)]
struct Instruction {
    pub typ: InstrType,
    pub val: i64,
}

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads instructions from a file.
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

/// Executes the instructions, returning the accumulator value (before any loop),
/// the instructions executed, and whether the program terminated (as opposed to
/// getting stuck in a loop).
fn execute(ins: &Vec<Instruction>) -> (i64, Vec<bool>, bool) {
    let mut visited = Vec::new();
    for _ in 0..ins.len() {
        visited.push(false);
    }
    let mut i: usize = 0;
    let mut acc: i64 = 0;
    while i < ins.len() && !visited[i] {
        visited[i as usize] = true;
        let cur_in = &ins[i];
        match cur_in.typ {
            InstrType::ACC => {
                acc += cur_in.val;
                i += 1;
            }
            InstrType::JMP => {
                if cur_in.val < 0 {
                    if ((-cur_in.val) as usize) > i {
                        // Cannot have a negative i.
                        break;
                    } else {
                        i -= (-cur_in.val) as usize;
                    }
                } else {
                    i += cur_in.val as usize;
                }
            }
            InstrType::NOP => {
                i += 1;
            }
        }
    }
    (acc, visited, i >= ins.len())
}

/// Gets the value of the accumulator as per the instruction set before any instructions repeat.
fn get_acc_before_repeats(ins: &Vec<Instruction>) -> i64 {
    let (acc_val, _, _) = execute(ins);
    acc_val
}

/// Changes a jmp or nop instruction so the program terminates, returning the final accumulator value.
fn repair_and_execute(ins: &Vec<Instruction>) -> Option<i64> {
    let (acc_val, executed, term) = execute(ins);
    if term {
        return Some(acc_val);
    }
    // Copy the intructions so we can mutate them freely.
    let mut ins: Vec<Instruction> = ins.iter().cloned().collect();

    // Try substituting each nop and jmp and see if the program executes.
    for (i, executed) in executed.iter().enumerate() {
        if !executed {
            continue;
        }
        let new_typ = match ins[i].typ {
            InstrType::JMP => InstrType::NOP,
            InstrType::NOP => InstrType::JMP,
            _ => continue,
        };
        let old_in = ins.remove(i);
        let new_in = Instruction {
            typ: new_typ,
            val: old_in.val,
        };
        ins.insert(i, new_in);
        let (acc_val, _, term) = execute(&ins);
        if term {
            return Some(acc_val);
        }
        ins.remove(i);
        ins.insert(i, old_in);
    }
    None
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
    let acc_val_no_repair = get_acc_before_repeats(&ins);
    println!("Accumulator value before repeats: {}", acc_val_no_repair);

    println!("\n==== Part 2 ====");
    match repair_and_execute(&ins) {
        Some(acc) => println!("Accumulator value after repairing: {}", acc),
        None => println!("No possible repair found"),
    }
}
