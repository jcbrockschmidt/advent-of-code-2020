use std::cmp;
use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

struct BoardingPass {
    pub row: u8,
    pub col: u8,
    pub seat_id: u16,
}

impl BoardingPass {
    pub fn new(row: u8, col: u8) -> BoardingPass {
        BoardingPass {
            row: row,
            col: col,
            seat_id: (row as u16) * 8 + (col as u16),
        }
    }
}

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Divides a 1D bound space on the left or right for binary space partitioning.
fn bsp_split(bounds: (u8, u8), left: bool) -> (u8, u8) {
    let (min, max) = bounds;
    let step = (max - min + 1) / 2;
    if left {
        (min, max - step)
    } else {
        (min + step, max)
    }
}

/// Reads boarding passes from a file.
fn read_passes<P>(filename: P) -> Result<Vec<BoardingPass>, String>
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

    let mut passes: Vec<BoardingPass> = Vec::new();
    for (i, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        if line.len() != 10 {
            return Err(format!("Bad pass on line {}", i + 1));
        }

        // Get the row.
        let mut row_bounds: (u8, u8) = (0, 127);
        for ch in line[..7].chars() {
            match ch {
                'F' => row_bounds = bsp_split(row_bounds, true),
                'B' => row_bounds = bsp_split(row_bounds, false),
                _ => return Err(format!("Bad character '{}' on line {}", ch, i + 1)),
            }
        }

        // Get the column.
        let mut col_bounds: (u8, u8) = (0, 7);
        for ch in line[7..].chars() {
            match ch {
                'L' => col_bounds = bsp_split(col_bounds, true),
                'R' => col_bounds = bsp_split(col_bounds, false),
                _ => return Err(format!("Bad character '{}' on line {}", ch, i + 1)),
            }
        }

        passes.push(BoardingPass::new(row_bounds.0, col_bounds.0));
    }
    Ok(passes)
}

/// Gets the highest seat ID in the collection of boarding passes.
fn get_highest_id(passes: &Vec<BoardingPass>) -> u16 {
    passes
        .iter()
        .fold(0, |max_id, bp| cmp::max(max_id, bp.seat_id))
}

/// Gets the missing boarding pass ID.
fn get_missing_pass(passes: &Vec<BoardingPass>) -> Option<u16> {
    let mut p_refs: Vec<&BoardingPass> = passes.iter().collect();
    p_refs.sort_by_key(|p| p.seat_id);
    let mut prev_id = p_refs[0].seat_id;
    for p in p_refs[1..].iter() {
        if p.seat_id != prev_id + 1 {
            return Some(p.seat_id - 1);
        }
        prev_id = p.seat_id;
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
    let passes = match read_passes(input_path) {
        Ok(ps) => ps,
        Err(e) => {
            eprintln!("Failed to read boarding passes: {}", e);
            process::exit(1);
        }
    };

    println!("\n==== Part 1 ====");
    let highest_id = get_highest_id(&passes);
    println!("Highest seat ID: {}", highest_id);

    println!("\n==== Part 2 ====");
    match get_missing_pass(&passes) {
        Some(id) => println!("Missing seat ID: {}", id),
        None => println!("No missing pass found"),
    }
}
