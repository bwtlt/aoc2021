use input_parser::read_lines;

struct Crab {
    position: i32,
}

fn compute_answer(crabs: &[Crab], constant_rate: bool) -> i32 {
    let mut min_fuel = i32::MAX;
    let max = crabs.iter().max_by(|x, y| x.position.cmp(&y.position)).unwrap().position;
    let min = crabs.iter().min_by(|x, y| x.position.cmp(&y.position)).unwrap().position;
    for position in min..=max {
        let mut fuel = 0;
        for crab in crabs {
            let steps = (crab.position - position).abs();
            if constant_rate {
                fuel += steps;
            } else {
                fuel += steps * (steps + 1)/2;
            }
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

fn main() {
    if let Ok(mut lines) = read_lines("./input.txt") {
        let crabs: Vec<Crab> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|n| Crab {
                position: n.parse::<i32>().unwrap(),
            })
            .collect();
        println!("Part1 answer: {}", compute_answer(&crabs, true));
        println!("Part2 answer: {}", compute_answer(&crabs, false));
    }
}
