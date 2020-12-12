use std::convert::TryFrom;
use std::env;
use std::path::Path;
use std::process;
use std::usize;
use utils::read_lines;

/// Directions ship can strafe in.
#[derive(Clone, Copy)]
enum StrafeDir {
    North,
    East,
    South,
    West,
}

/// Directions ship can rotate.
#[derive(Clone, Copy)]
enum RotateDir {
    Left,
    Right,
}

/// Types of movement instructions for a ship.
#[derive(Clone, Copy)]
enum InstrType {
    Strafe(StrafeDir),
    Rotate(RotateDir),
    Forward,
}

/// An instruction type it's magnitude.
#[derive(Clone, Copy)]
struct Instruction {
    pub typ: InstrType,
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
            'N' => InstrType::Strafe(StrafeDir::North),
            'E' => InstrType::Strafe(StrafeDir::East),
            'S' => InstrType::Strafe(StrafeDir::South),
            'W' => InstrType::Strafe(StrafeDir::West),
            'L' => InstrType::Rotate(RotateDir::Left),
            'R' => InstrType::Rotate(RotateDir::Right),
            'F' => InstrType::Forward,
            _ => {
                return Err(format!(
                    "Invalid instruction type \"{}\" on line {}",
                    ins_char,
                    i + 1
                ));
            }
        };
        let mut val: i32 = match line[1..].parse() {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to number on line {}", i + 1)),
        };
        if let InstrType::Rotate(_) = typ {
            val /= 90;
        }
        ins.push(Instruction { typ: typ, val: val });
    }
    Ok(ins)
}

/// Executes the ship's movement instructions and returns the final position difference.
/// Ship begins pointing east.
fn execute_instructions(ins: &Vec<Instruction>) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let ship_dirs: [StrafeDir; 4] = [
        StrafeDir::North,
        StrafeDir::East,
        StrafeDir::South,
        StrafeDir::West,
    ];
    // Ship starts by facing east.
    let mut rot_i: usize = 1;
    for i in ins.iter() {
        let mut strafe = None;
        match &i.typ {
            InstrType::Strafe(dir) => {
                strafe = Some(dir);
            }
            InstrType::Rotate(dir) => {
                let i_diff = match dir {
                    RotateDir::Left => -i.val,
                    RotateDir::Right => i.val,
                };
                let mut new_rot_i = (i32::try_from(rot_i).unwrap() + i_diff) % 4;
                if new_rot_i < 0 {
                    new_rot_i = 4 + new_rot_i;
                }
                rot_i = usize::try_from(new_rot_i).unwrap();
            }
            InstrType::Forward => {
                strafe = Some(&ship_dirs[rot_i]);
            }
        }
        if let Some(dir) = strafe {
            match dir {
                StrafeDir::North => y += i.val,
                StrafeDir::East => x += i.val,
                StrafeDir::South => y -= i.val,
                StrafeDir::West => x -= i.val,
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
