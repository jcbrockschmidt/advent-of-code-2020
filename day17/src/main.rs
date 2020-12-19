#[macro_use]
extern crate utils;

mod grid;

use std::env;
use std::path::Path;
use std::process;
use utils::read_lines;

use grid::Expand3DGrid;

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// Reads a grid from a file.
fn read_grid<P>(filename: P) -> Result<Expand3DGrid, String>
where
    P: AsRef<Path>,
{
    if !filename.as_ref().exists() {
        return Err("File does not exists".into());
    }
    let unread_lines = match read_lines(filename) {
        Ok(l) => l,
        Err(_) => return Err("Failed to read lines from file".into()),
    };

    let mut lines: Vec<String> = Vec::new();
    for (i, line_res) in unread_lines.enumerate() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.is_empty() {
            continue;
        }
        lines.push(line);
    }
    if lines.is_empty() {
        return Err("No lines found".into());
    }
    // Assume all lines are of the same length.
    let h = lines.len();
    let w = lines[0].len();
    let mut grid = Expand3DGrid::new((w, h, 1));
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid.toggle((x, y, 0));
            }
        }
    }
    Ok(grid)
}

/// Simulates a single cycle for a grid of cubes.
fn simulate_cycle(grid: &mut Expand3DGrid) {
    grid.expand_border();
    let (w, h, d) = grid.get_size();
    let mut to_toggle = Vec::new();
    for x in 0..w {
        for y in 0..h {
            for z in 0..d {
                let xyz = (x, y, z);
                let adj = grid.count_adj(xyz);
                if grid.has_cube(xyz) {
                    if adj < 2 || adj > 3 {
                        to_toggle.push(xyz);
                    }
                } else if adj == 3 {
                    to_toggle.push(xyz);
                }
            }
        }
    }
    for xyz in to_toggle.iter() {
        grid.toggle(*xyz);
    }
}

/// Simulates the a grid of cubes for `cycles` cycles.
fn simulate_grid(grid: &mut Expand3DGrid, cycles: usize) {
    for _ in 0..cycles {
        simulate_cycle(grid);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = String::from(&args[1]);
    let mut grid = timed_section!("Initialization", { read_grid(&input_path) }, |res| {
        match res {
            Ok(grid) => {
                println!("Initial grid:\n{}", grid);
                grid
            }
            Err(e) => {
                eprintln!("Failed to read grid: {}", e);
                process::exit(1);
            }
        }
    });

    timed_section!(
        "Part 1",
        {
            simulate_grid(&mut grid, 6);
            grid.count_cubes()
        },
        |cnt| {
            println!("Cubes left after 6 cycles: {}", cnt);
        }
    );
}
