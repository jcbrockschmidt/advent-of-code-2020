use std::path::Path;
use std::{env, process};
use utils::read_lines;

const TREE_CHAR: char = '#';

/// Prints usage statement for the executable.
fn usage(args: Vec<String>) {
    println!("Usage: {} input_file", args[0]);
}

/// A 2D horizontally repeating map of trees.
struct RepeatingTreeMap {
    pub w: usize,
    pub h: usize,
    grid: Vec<Vec<bool>>,
}

impl RepeatingTreeMap {
    /// Creates a blank map of width `w` and height `h`.
    pub fn new(w: usize, h: usize) -> RepeatingTreeMap {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        for _ in 0..h {
            let row = vec![false; w];
            grid.push(row);
        }
        RepeatingTreeMap {
            w: w,
            h: h,
            grid: grid,
        }
    }

    /// Sets a position as having a tree.
    pub fn set_tree(&mut self, x: usize, y: usize) {
        let x = x % self.w;
        let y = y % self.h;
        self.grid[y][x] = true;
    }

    /// Returns whether a position on the map contains a tree.
    pub fn contains_tree(&self, x: usize, y: usize) -> bool {
        let x = x % self.w;
        let y = y % self.h;
        self.grid[y][x]
    }
}

/// Reads a tree map from a file.
fn read_map<P>(filename: P) -> Result<RepeatingTreeMap, String>
where
    P: AsRef<Path>,
{
    if !filename.as_ref().exists() {
        return Err("File does not exists".into());
    }
    let lines_io = match read_lines(filename) {
        Ok(l) => l,
        Err(_) => return Err("Failed to read lines from file".into()),
    };
    let mut lines: Vec<Vec<char>> = Vec::new();
    for (i, line_res) in lines_io.enumerate() {
        let line: Vec<char> = match line_res {
            Ok(l) => l.chars().collect(),
            Err(_) => return Err(format!("Failed to read line {}", i + 1)),
        };
        if line.len() > 0 {
            lines.push(line);
        }
    }
    if lines.len() == 0 {
        return Err("No lines to read".into());
    }
    let mut map = RepeatingTreeMap::new(lines[0].len(), lines.len());
    for (y, line) in lines.iter().enumerate() {
        if line.len() != map.w {
            return Err("All non-empty lines must be of the same length".into());
        }
        for (x, c) in line.iter().enumerate() {
            if *c == TREE_CHAR {
                map.set_tree(x, y);
            }
        }
    }
    Ok(map)
}

/// Checks the number of trees hit given a slope trajectory.
fn check_slope(map: &RepeatingTreeMap, delta_x: usize, delta_y: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees: usize = 0;
    while y < map.h {
        if map.contains_tree(x, y) {
            trees += 1;
        }
        x += delta_x;
        y += delta_y;
    }
    trees
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(args);
        process::exit(1);
    }
    let input_path = Path::new(&args[1]);
    let map = match read_map(input_path) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Failed to load map: {}", e);
            process::exit(1);
        }
    };

    println!("==== Part 1 ====");
    let trees_hit = check_slope(&map, 3, 1);
    println!("{} trees hit", trees_hit);

    println!("\n==== Part 2 ====");
    let mut tree_prod = 1;
    tree_prod *= check_slope(&map, 1, 1);
    tree_prod *= check_slope(&map, 3, 1);
    tree_prod *= check_slope(&map, 5, 1);
    tree_prod *= check_slope(&map, 7, 1);
    tree_prod *= check_slope(&map, 1, 2);
    println!("product of all trees hit is {}", tree_prod);
}
