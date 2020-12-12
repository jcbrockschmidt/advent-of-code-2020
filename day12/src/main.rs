use std::convert::{TryFrom, TryInto};
use std::env;
use std::path::Path;
use std::process;
use std::usize;
use utils::read_lines;

enum InstructionType {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

struct Instruction {
    pub typ: InstructionType,
    pub val: i32,
}

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
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

    let mut ins: Vec<Instruction> = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        } else if line.len() <= 1 {
            return Err(format!("Line {} is too short", i + 1));
        }
        let ins_char = line.chars().nth(0).unwrap();
        let typ = match ins_char {
            'N' => InstructionType::North,
            'E' => InstructionType::East,
            'S' => InstructionType::South,
            'W' => InstructionType::West,
            'L' => InstructionType::Left,
            'R' => InstructionType::Right,
            'F' => InstructionType::Forward,
            _ => {
                return Err(format!(
                    "Invalid instruction type \"{}\" on line {}",
                    ins_char,
                    i + 1
                ));
            }
        };
        let val: i32 = match line[1..].parse() {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to number on line {}", i + 1)),
        };
        // TODO: check value for rotations.
        ins.push(Instruction { typ: typ, val: val });
    }
    Ok(ins)
}

/// TODO
fn execute_instructions(ins: &Vec<Instruction>) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // TODO: Make const
    let x_mults: [i32; 4] = [0, 1, 0, -1];
    let y_mults: [i32; 4] = [1, 0, -1, 0];
    // Ship starts by facing east.
    let mut rot_i: i32 = 1;
    for i in ins.iter() {
        match i.typ {
            InstructionType::North => {
                y += i.val;
            }
            InstructionType::East => {
                x += i.val;
            }
            InstructionType::South => {
                y -= i.val;
            }
            InstructionType::West => {
                x -= i.val;
            }
            InstructionType::Left => {
                let diff = -i.val / 90;
                rot_i = (rot_i + diff) % 4;
                if rot_i < 0 {
                    rot_i = 4 + rot_i;
                }
            }
            InstructionType::Right => {
                let diff = i.val / 90;
                rot_i = (rot_i + diff) % 4;
            }
            InstructionType::Forward => {
                x = x + i.val * x_mults[usize::try_from(rot_i).unwrap()];
                y = y + i.val * y_mults[usize::try_from(rot_i).unwrap()];
            }
        }
    }
    (x, y)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let ins = match read_instructions(&input_path) {
        Ok(ins) => ins,
        Err(e) => {
            eprintln!("Failed to read instructions: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let (final_x, final_y) = execute_instructions(&ins);
    println!(
        "Final Manhattan distance: {}",
        final_x.abs() + final_y.abs()
    );
}
