use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;

#[derive(Debug)]
struct Vent {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}
impl FromStr for Vent {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let re = Regex::new(r"(\d+,\d+) -> (\d+,\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let start = caps.get(1).unwrap().as_str().split(',').map(|s| s.parse::<u32>().unwrap_or_default()).collect::<Vec<u32>>();
        let end = caps.get(2).unwrap().as_str().split(',').map(|s| s.parse::<u32>().unwrap_or_default()).collect::<Vec<u32>>();
        Ok(Vent {
            x1: start[0],
            y1: start[1],
            x2: end[0],
            y2: end[1],
            })
    }
}

impl Vent {
    fn parse(s: &str) -> Option<Self> {
        let vent = Vent::from_str(s).unwrap();
        if vent.x1 == vent.x2 || vent.y1 == vent.y2 {
            return Some(vent);
        }
        None
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input_example.txt") {
        let vents = lines.filter_map(|line| Vent::parse(&line.unwrap()));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
