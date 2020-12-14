#[macro_use]
extern crate utils;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process;
use std::usize;
use utils::read_lines;

/// Single line of instruction.
enum MaskInstr {
    Mask(usize, usize, usize),
    Mem(usize, usize),
}

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file scope", args[0]);
}

/// Reads mask instructions from a file.
fn read_mask_ins<P>(filename: P) -> Result<Vec<MaskInstr>, String>
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

    let mut ins: Vec<MaskInstr> = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.split(' ').collect();
        if tokens.len() != 3 {
            return Err(format!("Bad line {}", i + 1));
        }
        if tokens[0] == "mask" {
            let mut zeros_mask: usize = 0;
            let mut ones_mask: usize = 0;
            let mut x_mask: usize = 0;
            for (i, ch) in tokens[2].chars().rev().enumerate() {
                let bit: usize = 1 << i;
                match ch {
                    '0' => zeros_mask |= bit,
                    '1' => ones_mask |= bit,
                    'X' => x_mask |= bit,
                    _ => {
                        return Err(format!(
                            "Unrecognized character \"{}\" on line {}",
                            ch,
                            i + 1
                        ))
                    }
                }
            }
            ins.push(MaskInstr::Mask(zeros_mask, ones_mask, x_mask));
        } else if &tokens[0][..3] == "mem" {
            let index_str = &tokens[0][4..tokens[0].len() - 1];
            let addr: usize = match index_str.parse() {
                Ok(i) => i,
                Err(_) => return Err(format!("Bad index \"{}\" on line {}", index_str, i + 1)),
            };
            let val: usize = match tokens[2].parse() {
                Ok(i) => i,
                Err(_) => return Err(format!("Bad value on line {}", i + 1)),
            };
            ins.push(MaskInstr::Mem(addr, val));
        } else {
            return Err(format!("Bad line {}", i + 1));
        }
    }
    Ok(ins)
}

/// Use method 1 to read the mask instructions. Returns the sum of all values left in memory.
fn exec_ins_method1(ins: &Vec<MaskInstr>) -> usize {
    let mut and_mask = usize::MAX;
    let mut or_mask: usize = 0;
    let mut values: HashMap<usize, usize> = HashMap::new();
    for i in ins {
        match i {
            MaskInstr::Mask(zeros, ones, _) => {
                and_mask = !zeros;
                or_mask = *ones;
            }
            MaskInstr::Mem(addr, v) => {
                values.insert(*addr, (v & and_mask) | or_mask);
            }
        }
    }
    values.iter().fold(0, |sum, (_, v)| sum + v)
}

/// Use method 2 to read the mask instructions. Returns the sum of all values left in memory.
fn exec_ins_method2(ins: &Vec<MaskInstr>) -> usize {
    let mut unchanged_mask = usize::MAX;
    let mut change_mask: usize = 0;
    let mut float_mask: usize = 0;
    let mut values: HashMap<usize, usize> = HashMap::new();
    for i in ins {
        match i {
            MaskInstr::Mask(zeros, ones, float) => {
                unchanged_mask = *zeros;
                change_mask = *ones;
                float_mask = *float;
            }
            MaskInstr::Mem(addr, v) => {
                let base_addr: usize = (addr & unchanged_mask) | change_mask;
                let mut num_combos: usize = 0;
                for i in 0..36 {
                    if (float_mask >> i) & 1 == 1 {
                        if num_combos == 0 {
                            num_combos = 2;
                        } else {
                            num_combos *= 2;
                        }
                    }
                }
                for combo in 0..num_combos {
                    let mut new_addr = base_addr;
                    let mut one_i = 0;
                    for i in 0..36 {
                        if (float_mask >> i) & 1 == 1 {
                            if combo >> one_i & 1 == 1 {
                                new_addr |= 1 << i;
                            }
                            one_i += 1;
                        }
                    }
                    values.insert(new_addr, *v);
                }
            }
        }
    }
    values.iter().fold(0, |sum, (_, v)| sum + v)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let ins = timed_section!("Initialization", { read_mask_ins(&input_path) }, |v| {
        match v {
            Ok(ins) => ins,
            Err(e) => {
                eprintln!("Failed to read mask instruction data: {}", e);
                process::exit(1);
            }
        }
    });

    timed_section!("Part 1", { exec_ins_method1(&ins) }, |v| {
        println!("part 1: {}", v);
    });

    timed_section!("Part 2", { exec_ins_method2(&ins) }, |v| {
        println!("part 2: {}", v);
    });
}
