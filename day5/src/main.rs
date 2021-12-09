use regex::Regex;
use input_parser::read_lines;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Vent {
    points: Vec<Point>,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Other,
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
        let horizontal = start.y == end.y;
        let vertical = start.x == end.x;
        let diagonal =
            (start.x as i32 - end.x as i32).abs() == (start.y as i32 - end.y as i32).abs();
        if !horizontal && !vertical && !diagonal {
            return None;
        }
        let direction = if horizontal {
            if start.x < end.x {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if vertical {
            if start.y < end.y {
                Direction::Down
            } else {
                Direction::Up
            }
        } else if diagonal {
            if start.x < end.x {
                if start.y < end.y {
                    Direction::DownRight
                } else {
                    Direction::UpRight
                }
            } else if start.y < end.y {
                Direction::DownLeft
            } else {
                Direction::UpLeft
            }
        } else {
            Direction::Other
        };
        let mut vent = Vent {
            points: Vec::new(),
            direction,
        };
        vent.points.push(start);
        let mut point = start;
        let x_step = match direction {
            Direction::Left | Direction::UpLeft | Direction::DownLeft => -1,
            Direction::Right | Direction::UpRight | Direction::DownRight => 1,
            _ => 0,
        };
        let y_step = match direction {
            Direction::Up | Direction::UpLeft | Direction::UpRight => -1,
            Direction::Down | Direction::DownLeft | Direction::DownRight => 1,
            _ => 0,
        };
        while point != end {
            point.x = (point.x as i32 + x_step) as u32;
            point.y = (point.y as i32 + y_step) as u32;
            vent.points.push(point);
        }
        Some(vent)
    }

    fn is_diagonal(&self) -> bool {
        self.direction == Direction::UpLeft
            || self.direction == Direction::UpRight
            || self.direction == Direction::DownLeft
            || self.direction == Direction::DownRight
    }
}

const ARRAY_SIZE: usize = 1000;

fn compute_answer(vents: &[Vent], include_diagonals: bool) -> u32 {
    let mut grid: Vec<Vec<Option<u32>>> = vec![vec![None; ARRAY_SIZE]; ARRAY_SIZE];
    vents.iter().for_each(|vent| {
        if vent.is_diagonal() && !include_diagonals {
            return;
        }
        vent.points
            .iter()
            .for_each(|point| match grid[point.y as usize][point.x as usize] {
                Some(v) => grid[point.y as usize][point.x as usize] = Some(v + 1),
                None => grid[point.y as usize][point.x as usize] = Some(1),
            })
    });
    grid.iter().flatten().fold(0, |acc, &x| match x {
        Some(n) if n >= 2 => acc + 1,
        _ => acc,
    })
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let vents = lines
            .filter_map(|line| Vent::parse(&line.unwrap()))
            .collect::<Vec<Vent>>();
        println!("Part1 answer {}", compute_answer(&vents, false));
        println!("Part2 answer {}", compute_answer(&vents, true));
    }
}
