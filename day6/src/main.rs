use input_parser::read_lines;

#[derive(Debug)]
struct LanternFish {
    timer: u8,
    number: u32,
}

fn print_fishes(fishes: &[u64]) -> String {
    let mut fishes_string = String::new();
    let mut sep = "";
    for timer in 0..=8 {
        fishes_string.push_str(sep);
        fishes_string.push_str(&timer.to_string());
        fishes_string.push(':');
        fishes_string.push_str(&fishes.get(timer as usize).unwrap_or(&0).to_string());
        sep = ", ";
    }
    fishes_string
}

const DEBUG: bool = false;

fn main() {
    if let Ok(mut lines) = read_lines("./input.txt") {
        let mut fishes = vec![0u64; 9];
        for item in lines.next().unwrap().unwrap().split(',') {
            let age = item.parse::<u32>().unwrap();
            fishes[age as usize] += 1;
        }
        if DEBUG { println!("Initial state:\t{}", print_fishes(&fishes)) };
        for day in 1..=256 {
            let new_fishes = fishes[0];
            for timer in 0..=8 {
                let next;
                match timer {
                    0 => {
                        next = 7;
                    },
                    _ => {
                        next = timer - 1;
                    },
                }
                if fishes[timer] > 0 {fishes[next] += fishes[timer]; fishes[timer] = 0; };
            }
            fishes[8] = new_fishes;
            if DEBUG { println!("After\t{} day:\t{}", day, print_fishes(&fishes)) };
        }
        println!("Total: {} fishes", fishes.iter().sum::<u64>());
    }
}
