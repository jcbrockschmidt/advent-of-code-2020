use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

const SEC_UNITS: [&str; 4] = ["s", "ms", "μs", "ns"];

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

/// Formats a duration as human-readable. Scales between microseconds, nanoseconds,
/// milliseconds, and seconds depending on the duration, and includes the unit.
///
/// WARNING: Floats are used for internal arithmetic, so the diplayed number is subject to floating point inprecisions. However this should only be a problem for
///
/// # Arguments
///
///  * `d` - Duration to format.
///
/// # Returns
///
/// The string formatted duration.
pub fn display_duration(d: Duration) -> String {
    let mut t = d.as_secs_f64();
    let mut unit = SEC_UNITS[0];
    for u in SEC_UNITS.iter().skip(1) {
        if t >= 1.0 {
            break;
        }
        t *= 1000.0;
        unit = u;
    }
    format!("{:.2}{}", t, unit)
}

#[macro_export]
macro_rules! timed_section {
    ($title:expr, $compute:block, $after:expr) => {{
        println!("==== {} ====", $title);
        let start = std::time::Instant::now();
        let values = $compute;
        let elapsed = start.elapsed();
        let ret = $after(values);
        println!("---- finished in {}\n", utils::display_duration(elapsed));
        ret
    }};
}
