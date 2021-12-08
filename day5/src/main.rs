use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Vent {
    points: Vec<Point>,
}

impl Vent {
    fn parse(s: &str) -> Option<Vent> {
        let re = Regex::new(r"(\d+,\d+) -> (\d+,\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let start = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap_or_default())
            .collect::<Vec<u32>>();
        let start = Point {
            x: start[0],
            y: start[1],
        };
        let end = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap_or_default())
            .collect::<Vec<u32>>();
        let end = Point {
            x: end[0],
            y: end[1],
        };
        let horizontal = start.x == end.x;
        let vertical = start.y == end.y;
        let positive = (horizontal && start.y < end.y) || (vertical && start.x < end.x);
        if !horizontal && !vertical {
            return None;
        }
        let mut vent = Vent { points: Vec::new() };
        vent.points.push(start);
        let mut point = start;
        let step: i32 = if positive { 1 } else { -1 };
        while point != end {
            if horizontal {
                point.y = (point.y as i32 + step) as u32;
            } else if vertical {
                point.x = (point.x as i32 + step) as u32;
            }
            vent.points.push(point);
        }
        Some(vent)
    }
}

const ARRAY_SIZE: usize = 1000;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let vents = lines.filter_map(|line| Vent::parse(&line.unwrap()));
        let mut grid: [[Option<u32>; ARRAY_SIZE]; ARRAY_SIZE] = [[None; ARRAY_SIZE]; ARRAY_SIZE];
        vents.for_each(|vent| {
            vent.points
                .iter()
                .for_each(|point| match grid[point.x as usize][point.y as usize] {
                    Some(v) => grid[point.x as usize][point.y as usize] = Some(v + 1),
                    None => grid[point.x as usize][point.y as usize] = Some(1),
                })
        });
        let answer = grid.iter().flatten().fold(0, |acc, &x| {
            match x {
                Some(n) if n >=2 => acc+1,
                _ => acc,
            }
        });
        println!("Part1 answer {}", answer);
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
