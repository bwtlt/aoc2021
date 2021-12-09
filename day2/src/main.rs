use std::str::FromStr;
use std::num::ParseIntError;
use input_parser::read_lines;

enum Command {
    None,
    Down(i32),
    Up(i32),
    Forward(i32),
}
impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut command = s.split_whitespace();
        let function = command.next().unwrap();
        let value = command.next().unwrap().parse::<i32>().unwrap();
        let move_command = match function {
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            "forward" => Command::Forward(value),
            _ => Command::None,
        };
        Ok(move_command)
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        struct SubmarinePosition {
            horizontal: i32,
            depth: i32,
        }

        let mut part1 = SubmarinePosition{horizontal: 0, depth: 0};
        let mut part2 = SubmarinePosition{horizontal: 0, depth: 0};
        let mut aim: i32 = 0;
        lines.map(|line| Command::from_str(line.unwrap().as_str())).for_each(
            |command| 
            match command.unwrap() {
                Command::Down(x) => {part1.depth += x; aim+=x;},
                Command::Up(x) => {part1.depth -= x; aim-=x;},
                Command::Forward(x) => {part1.horizontal += x; part2.horizontal+=x;
                    part2.depth+=aim*x;},
                _ => (),
            }
        );
        println!("Part 1 solution: {}", part1.horizontal * part1.depth);
        println!("Part 2 solution: {}", part2.horizontal * part2.depth);
    }
}
