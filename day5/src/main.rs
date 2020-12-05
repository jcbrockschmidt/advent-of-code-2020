use std::cmp;
use std::path::Path;
use std::{env, process};
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
        // let mut min_row = 0;
        // let mut max_row = 127;
        for ch in line[..7].chars() {
            // let step = (max_row - min_row + 1) / 2;
            // match ch {
            //     'F' => max_row = max_row - step,
            //     'B' => min_row = min_row + step,
            //     _ => return Err(format!("Bad character '{}' on line {}", ch, i + 1)),
            // }
            match ch {
                'F' => row_bounds = bsp_split(row_bounds, true),
                'B' => row_bounds = bsp_split(row_bounds, false),
                _ => return Err(format!("Bad character '{}' on line {}", ch, i + 1)),
            }
        }

        // Get the column.
        let mut min_col = 0;
        let mut max_col = 7;
        for ch in line[7..].chars() {
            let step = (max_col - min_col + 1) / 2;
            match ch {
                'L' => max_col = max_col - step,
                'R' => min_col = min_col + step,
                _ => return Err(format!("Bad character '{}' on line {}", ch, i + 1)),
            }
        }

        //passes.push(BoardingPass::new(min_row, min_col));
        passes.push(BoardingPass::new(row_bounds.0, min_col));
    }
    Ok(passes)
}

/// Gets the highest seat ID in the collection of boarding passes.
fn get_highest_id(passes: &Vec<BoardingPass>) -> u16 {
    passes
        .iter()
        .fold(0, |max_id, bp| cmp::max(max_id, bp.seat_id))
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
}
