use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Gets an iterator over the lines in a file.
///
/// # Arguments
///
///  * `filename` - Path to the file.
///
/// # Returns
///
/// An iterator over the file's lines.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
