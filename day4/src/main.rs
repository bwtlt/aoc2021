use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Copy)]
struct Number {
    number: u32,
    drawn: bool,
}

struct Grid {
    numbers: Vec<Vec<Number>>,
    won: bool,
}

fn main() {
    if let Ok(mut lines) = read_lines("./input_example.txt") {
        let drawn_numbers: Vec<u32> = lines
            .next()
            .unwrap()
            .unwrap_or_default()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let gridlines: Vec<String> = lines
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .collect();
        let mut grids: Vec<Vec<Vec<Number>>> = gridlines
            .chunks(5)
            .map(|lines| {
                lines
                    .iter()
                    .map(|line| {
                        line.trim()
                            .split_whitespace()
                            .map(|n| Number {
                                number: n.parse::<u32>().unwrap(),
                                drawn: false,
                            })
                            .collect::<Vec<Number>>()
                    })
                    .collect::<Vec<Vec<Number>>>()
            })
            .collect::<Vec<Vec<Vec<Number>>>>();

        println!("Part1");
        let mut won = false;
        for number in &drawn_numbers {
            grids.iter_mut().for_each(|grid| {
                if let Some(mut found) = grid.iter_mut().flatten().find(|x| x.number == *number) {
                    found.drawn = true;
                }
            });
            // check grids
            grids.iter().for_each(|grid| {
                // no need to check if less than 5 numbers drawn are present
                if grid.iter().flatten().filter(|number| number.drawn).count() >= 5 {
                    //check rows
                    grid.iter().for_each(|line| {
                        if is_complete(line) {
                            compute_result(*number, grid);
                            won = true;
                        }
                    });
                    //check column
                    if !won {
                        for i in 0..5 {
                            let mut column = Vec::new();
                            for item in grid.iter().take(5) {
                                column.push(item[i]);
                            }
                            if is_complete(&column) {
                                compute_result(*number, grid);
                                won = true;
                            }
                        }
                    }
                }
            });
            if won {
                break;
            }
        }

        println!("Part2");
        let mut won = false;
        for number in &drawn_numbers {
            grids.iter_mut().for_each(|grid| {
                if let Some(mut found) = grid.iter_mut().flatten().find(|x| x.number == *number) {
                    found.drawn = true;
                }
            });
            // check grids
            grids.iter().for_each(|grid| {
                // no need to check if less than 5 numbers drawn are present
                if grid.iter().flatten().filter(|number| number.drawn).count() >= 5 {
                    //check rows
                    grid.iter().for_each(|line| {
                        if is_complete(line) {
                            won = true;
                        }
                    });
                    //check column
                    if !won {
                        for i in 0..5 {
                            let mut column = Vec::new();
                            for item in grid.iter().take(5) {
                                column.push(item[i]);
                            }
                            if is_complete(&column) {
                                won = true;
                            }
                        }
                    }
                }
            });
            if won {
                break;
            }
        }

    }
}

fn is_complete(line: &[Number]) -> bool {
    return line.iter().all(|number| number.drawn);
}

fn compute_result(last_drawn: u32, grid: &[Vec<Number>]) {
    let sum: u32 = grid
        .iter()
        .flatten()
        .filter(|number| !number.drawn)
        .map(|number| number.number)
        .sum();
    println!("Answer: {}", sum * last_drawn);
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
