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
        let val: i32 = match line[1..].parse() {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to number on line {}", i + 1)),
        };
        ins.push(Instruction { typ: typ, val: val });
    }
    Ok(ins)
}

/// Executes the ship's movement instructions and returns the final position difference.
/// Ship begins pointing east.
fn exec_ins_ship(ins: &Vec<Instruction>) -> (i32, i32) {
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
                    RotateDir::Left => -i.val / 90,
                    RotateDir::Right => i.val / 90,
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

/// Rotates a point (`x`, `y`) `degree` degrees about the origin.
fn rotate_point(x: i32, y: i32, degree: i32) -> (i32, i32) {
    let cos: [i32; 4] = [1, 0, -1, 0];
    let sin: [i32; 4] = [0, 1, 0, -1];
    let mut step = (degree / 90) % 4;
    if step < 0 {
        step = 4 + step;
    }
    let i = usize::try_from(step).unwrap();
    let new_x = x * cos[i] - y * sin[i];
    let new_y = x * sin[i] + y * cos[i];
    (new_x, new_y)
}

/// Executes the ship's movement instructions using the waypoing method and returns the
/// final position difference.
fn exec_ins_waypoint(ins: &Vec<Instruction>) -> (i32, i32) {
    let mut ship_x: i32 = 0;
    let mut ship_y: i32 = 0;
    // Position of the waypoint relative to the ship.
    let mut wp_x: i32 = 10;
    let mut wp_y: i32 = 1;
    for i in ins.iter() {
        match &i.typ {
            InstrType::Strafe(dir) => match dir {
                StrafeDir::North => wp_y += i.val,
                StrafeDir::East => wp_x += i.val,
                StrafeDir::South => wp_y -= i.val,
                StrafeDir::West => wp_x -= i.val,
            },
            InstrType::Rotate(dir) => {
                let degrees = match dir {
                    RotateDir::Left => i.val,
                    RotateDir::Right => -i.val,
                };
                let (new_wp_x, new_wp_y) = rotate_point(wp_x, wp_y, degrees);
                wp_x = new_wp_x;
                wp_y = new_wp_y;
            }
            InstrType::Forward => {
                ship_x += wp_x * i.val;
                ship_y += wp_y * i.val;
            }
        }
    }
    (ship_x, ship_y)
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
    let (final_x, final_y) = exec_ins_ship(&ins);
    println!(
        "Final Manhattan distance using ship only: {}",
        final_x.abs() + final_y.abs()
    );

    println!("\n==== Part 2 ====");
    let (final_wp_x, final_wp_y) = exec_ins_waypoint(&ins);
    println!(
        "Final Manhattan distance using a waypoint: {}",
        final_wp_x.abs() + final_wp_y.abs()
    );
}
